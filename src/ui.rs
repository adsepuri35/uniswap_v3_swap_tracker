use std::io;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal};
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Constraint},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Table, Row, Cell},
    DefaultTerminal, Frame,
};
use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use tokio::time::Duration;
use std::cmp::min;

//file imports
use crate::poollnfo::PoolInfo;

#[derive(Debug)]
pub struct TerminalUI {
    exit: bool,
    pools: Vec<PoolInfo>,
    total_swaps: usize,
    rx: Option<Receiver<Vec<PoolInfo>>>,
    scroll_offset: usize,
    selected_pool_index: usize,
}

impl Default for TerminalUI {
    fn default() -> Self {
        Self {
            exit: false,
            pools: Vec::new(),
            total_swaps: 0,
            rx: None,
            scroll_offset: 0,
            selected_pool_index: 0,
        }
    }
}

impl TerminalUI {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            // First, check for events
            let handle_events = if let Some(rx) = &mut self.rx {
                // Try to receive without blocking UI
                event::poll(std::time::Duration::from_millis(10))?
            } else {
                // Always check events if we don't have a channel
                self.handle_events()?;
                false // Already handled events
            };
            
            // Handle events if needed (separate from channel checking)
            if handle_events {
                self.handle_events()?;
            }
            
            // Now check for channel updates
            if let Some(rx) = &mut self.rx {
                // Check channel for updates
                match rx.try_recv() {
                    Ok(pools) => {
                        // Update the UI with new pool data
                        self.pools = pools;
                        self.total_swaps = self.pools.iter().map(|p| p.get_swap_count()).sum();
                    }
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                        // No data available, that's fine
                    }
                    Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                        // Channel closed, might want to handle this
                        self.rx = None;
                        // println!("Channel disconnected");
                        // self.status_message = "Backend disconnected. Data will not update.".to_string();
                    }
                }
            }
            
            // Draw UI
            terminal.draw(|frame| self.draw(frame))?;
            
            // Small sleep to prevent CPU spinning
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),


            // Add scroll controls
            KeyCode::Up | KeyCode::Char('k') => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                    self.selected_pool_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // Only scroll if there are more pools to show
                if self.scroll_offset < self.pools.len().saturating_sub(1) {
                    self.scroll_offset += 1;
                    self.selected_pool_index += 1;
                }
            }
            KeyCode::PageUp => {
                // Scroll a page up (10 lines at a time)
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
            }
            KeyCode::PageDown => {
                // Scroll a page down (10 lines at a time)
                let max_scroll = self.pools.len().saturating_sub(1);
                self.scroll_offset = (self.scroll_offset + 10).min(max_scroll);
            }
            KeyCode::Home => {
                // Scroll to the top
                self.scroll_offset = 0;
            }
            KeyCode::End => {
                // Scroll to the bottom
                self.scroll_offset = self.pools.len().saturating_sub(1);
            }


            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn is_exiting(&self) -> bool {
        self.exit
    }

    pub fn with_receiver(rx: Receiver<Vec<PoolInfo>>) -> Self {
        Self {
            exit: false,
            pools: Vec::new(),
            total_swaps: 0,
            rx: Some(rx),
            scroll_offset: 0,
            selected_pool_index: 0,
        }
    }

}

impl Widget for &TerminalUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Up ".into(),
            "<↑>".blue().bold(),
            " Down ".into(),
            "<↓>".blue().bold(),
            " Page Up ".into(),
            "<PgUp>".blue().bold(),
            " Page Down ".into(),
            "<PgDn>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let title = Line::from(" Uniswap Swap Tracker ".magenta().bold());

        // Create a block for the UI
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_type(ratatui::widgets::BorderType::Rounded);

        // Render the block in the area
        block.clone().render(area, buf);

        // Calculate inner area for content
        let inner_area = block.inner(area);

        // Create header with total stats
        let header = format!(" Pools Tracked: {} | Swaps Tracked: {}", self.pools.len(), self.total_swaps);
        let header_text = Text::from(header);

        // Render header
        Paragraph::new(header_text)
            .render(Rect::new(inner_area.x, inner_area.y, inner_area.width, 1), buf);

        // Split the inner area into two panes: left (Pools) and right (Prices)
        let panes = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // Pools block takes 70% of the width
                Constraint::Percentage(30), // Prices block takes 30% of the width
            ])
            .split(Rect::new(
                inner_area.x,
                inner_area.y + 2, // +2 to leave space after header
                inner_area.width,
                inner_area.height.saturating_sub(2),
            ));

        let pools_area = panes[0];
        let right_area = panes[1];

        // Render Pools block
        let mut sorted_pools = self.pools.clone();
        sorted_pools.sort_by(|a, b| b.get_swap_count().cmp(&a.get_swap_count()));

        let pool_count = self.pools.len();
        let max_visible = pools_area.height as usize;
        let start_idx: usize = if pool_count <= max_visible {
            0
        } else {
            let max_start = pool_count.saturating_sub(max_visible);
            self.scroll_offset.min(max_start)
        };

        let visible_pools = &sorted_pools[start_idx..min(start_idx + max_visible, pool_count)];

        let headers = Row::new(vec![
            Cell::from("ID"),
            Cell::from("Pool Name"),
            Cell::from("Protocol"),
            Cell::from("Fee"),
            Cell::from("Swaps"),
            Cell::from("Price"),
            Cell::from("Liquidity"),
            Cell::from("Lower Tick"),
            Cell::from("Upper Tick"),
            Cell::from("APR"),
            Cell::from("Volume (ETH)"),
        ])
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));

        let rows: Vec<Row> = visible_pools.iter().enumerate().map(|(i, pool)| {
            let price_display = {
                let price = pool.get_current_price();
                let price_text = if price == 0.0 {
                    "Price unknown".to_string()
                } else if price >= 1_000_000.0 {
                    format!("${:.2}M", price / 1_000_000.0)
                } else if price >= 1000.0 {
                    format!("${:.2}", price)
                } else if price >= 1.0 {
                    format!("${:.4}", price)
                } else if price >= 0.01 {
                    format!("${:.6}", price)
                } else if price >= 0.00001 {
                    format!("${:.8}", price)
                } else {
                    format!("${:.10}", price)
                };

                // Apply conditional styling
                let color = pool.get_price_change_color();
                Cell::from(price_text).style(ratatui::style::Style::default().fg(color))
            };

            let liquidity_display = {
                let liquidity = pool.get_liquidity();
                if liquidity >= 1_000_000_000_000_000_000 {
                    format!("{:.2}Q", liquidity as f64 / 1_000_000_000_000_000_000.0)
                } else if liquidity >= 1_000_000_000_000_000 {
                    format!("{:.2}Qd", liquidity as f64 / 1_000_000_000_000_000.0)
                } else if liquidity >= 1_000_000_000_000 {
                    format!("{:.2}T", liquidity as f64 / 1_000_000_000_000.0)
                } else if liquidity >= 1_000_000_000 {
                    format!("{:.2}B", liquidity as f64 / 1_000_000_000.0)
                } else if liquidity >= 1_000_000 {
                    format!("{:.2}M", liquidity as f64 / 1_000_000.0)
                } else if liquidity >= 1_000 {
                    format!("{:.2}K", liquidity as f64 / 1_000.0)
                } else {
                    format!("{}", liquidity)
                }
            };

            let (lower_tick, upper_tick) = pool.get_tick_range();

            fn format_volume(volume: f64) -> String {
                if volume >= 1_000_000_000_000_000_000.0 {
                    format!("{:.2}Qd", volume / 1_000_000_000_000_000_000.0)
                } else if volume >= 1_000_000_000_000_000.0 {
                    format!("{:.2}Q", volume / 1_000_000_000_000_000.0)
                } else if volume >= 1_000_000_000_000.0 {
                    format!("{:.2}T", volume / 1_000_000_000_000.0)
                } else if volume >= 1_000_000_000.0 {
                    format!("{:.2}B", volume / 1_000_000_000.0)
                } else if volume >= 1_000_000.0 {
                    format!("{:.2}M", volume / 1_000_000.0)
                } else if volume >= 1_000.0 {
                    format!("{:.2}K", volume / 1_000.0)
                } else if volume > 0.0 {
                    // For small non-zero values, display without trailing decimals
                    if volume.fract() == 0.0 {
                        format!("{:.0}", volume) // No decimal places for whole numbers
                    } else {
                        format!("{:.2}", volume) // Two decimal places for fractional values
                    }
                } else {
                    "0".to_string() // Display "0" for zero values
                }
            }

            let is_selected = start_idx + i == self.selected_pool_index;
            let style = if is_selected {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD)
            } else {
                ratatui::style::Style::default()
            };

            Row::new(vec![
                Cell::from(format!("{}", start_idx + i + 1)).style(style), // Index
                Cell::from(pool.get_pool_name().to_string()).style(style), // Pool Name
                Cell::from("v3").style(style),
                Cell::from(format!("{:.2}%", pool.get_fee_percent())).style(style), // Fee
                Cell::from(format!("{}", pool.get_swap_count())).style(style), // Swaps
                price_display.style(style), // Price
                Cell::from(liquidity_display).style(style), // Liquidity
                Cell::from(format!("{}", lower_tick)).style(style), // Lower Tick
                Cell::from(format!("{}", upper_tick)).style(style), // Upper Tick
                Cell::from(format!("{:.2}%", pool.get_current_apr())).style(style), // APR
                Cell::from(format_volume(pool.get_volume())).style(style), // Volume
            ])
        }).collect();

        let table = Table::new(
            rows,
            vec![
                Constraint::Length(6),
                Constraint::Length(20),
                Constraint::Length(13),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(20),
                Constraint::Length(20),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Length(15),
            ],
        )
        .header(headers)
        .block(
            Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title(" Pools "),
        );

        table.render(pools_area, buf);

        // Render right block
        let right_block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(" Track Swaps ");

        // Render swaps for the selected pool
        if !self.pools.is_empty() {
            let selected_pool = &self.pools[self.selected_pool_index]; // Get the selected pool
            let swap_store = selected_pool.get_swap_store(); // Retrieve the swap events

            let headers = Row::new(vec![
                Cell::from("Timestamp"),
                Cell::from("Amount0"),
                Cell::from("Amount1"),
            ])
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));

            // If there are no swaps, display an empty row
            let rows: Vec<Row> = if swap_store.is_empty() {
                vec![Row::new(vec![
                    Cell::from("No swaps available"),
                    Cell::from(""),
                    Cell::from(""),
                ])]
            } else {
                swap_store.iter().map(|(swap, timestamp)| {
                    Row::new(vec![
                        Cell::from(format!("{}", timestamp)), // Timestamp
                        Cell::from(format!("{}", swap.amount0)), // Amount0
                        Cell::from(format!("{}", swap.amount1)), // Amount1
                    ])
                }).collect()
            };

            let swaps_table = Table::new(
                rows,
                vec![
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Length(20),
                ],
            )
            .header(headers)
            .block(
                Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title(format!(" Swaps for Pool: {} ", selected_pool.get_pool_name())),
            );

            swaps_table.render(right_area, buf);
        } else {
            // If no pools are available, render an empty swaps table
            let headers = Row::new(vec![
                Cell::from("Timestamp"),
                Cell::from("Amount0"),
                Cell::from("Amount1"),
            ])
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));

            let rows = vec![Row::new(vec![
                Cell::from("No pools available"),
                Cell::from(""),
                Cell::from(""),
            ])];

            let swaps_table = Table::new(
                rows,
                vec![
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Length(20),
                ],
            )
            .header(headers)
            .block(
                Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title(" Track Swaps "),
            );

            swaps_table.render(right_area, buf);
        }

    }
}
