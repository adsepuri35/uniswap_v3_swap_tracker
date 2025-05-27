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
}

impl Default for TerminalUI {
    fn default() -> Self {
        Self {
            exit: false,
            pools: Vec::new(),
            total_swaps: 0,
            rx: None,
            scroll_offset: 0,
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
                        self.total_swaps = self.pools.iter().map(|p| *p.get_swap_count()).sum();
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
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // Only scroll if there are more pools to show
                if self.scroll_offset < self.pools.len().saturating_sub(1) {
                    self.scroll_offset += 1;
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
            "<Q>".blue().bold(),
        ]);

        let title = Line::from(" Uniswap Swap Tracker ".bold());

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
        let header = format!("Total Pools: {} | Total Swaps: {}", self.pools.len(), self.total_swaps);
        let header_text = Text::from(header);

        // Render header
        Paragraph::new(header_text)
            .render(Rect::new(inner_area.x, inner_area.y, inner_area.width, 1), buf);

        // Calculate available space for pool list
        let list_area = Rect::new(
            inner_area.x,
            inner_area.y + 2, // +2 to leave space after header
            inner_area.width,
            inner_area.height.saturating_sub(2),
        );

        // Determine visible range of pools based on scroll position
        let pool_count = self.pools.len();
        let max_visible = list_area.height as usize;
        let start_idx: usize = if pool_count <= max_visible {
            0 // Show from beginning if all fit
        } else {
            let max_start = pool_count.saturating_sub(max_visible);
            self.scroll_offset.min(max_start)
        };

        // Create a visible slice of pools
        let visible_pools = &self.pools[start_idx..min(start_idx + max_visible, pool_count)];

        // Create table headers
        let headers = Row::new(vec![
            Cell::from("Index"),
            Cell::from("Pool Name"),
            Cell::from("Fee"),
            Cell::from("Swaps"),
            Cell::from("Price"),
            Cell::from("Liquidity"),
            Cell::from("Lower Tick"),
            Cell::from("Upper Tick"),
        ])
        .style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD));

        // Create rows for each pool
        let rows: Vec<Row> = visible_pools.iter().enumerate().map(|(i, pool)| {
            let price_display = {
                let price = pool.get_current_price();
                if price == 0.0 {
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
                }
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

            Row::new(vec![
                Cell::from(format!("{}", start_idx + i + 1)), // Index
                Cell::from(pool.get_pool_name().to_string()), // Pool Name
                Cell::from(format!("{:.2}%", pool.get_fee_percent())), // Fee
                Cell::from(format!("{}", pool.get_swap_count())), // Swaps
                Cell::from(price_display), // Price
                Cell::from(liquidity_display), // Liquidity
                Cell::from(format!("{}", lower_tick)), // Lower Tick
                Cell::from(format!("{}", upper_tick)), // Upper Tick
            ])
        }).collect();

        // Create the table
        let table = Table::new(
            rows, // Rows for the table
            vec![
                Constraint::Length(6),  // Index
                Constraint::Length(30), // Pool Name
                Constraint::Length(10), // Fee
                Constraint::Length(10), // Swaps
                Constraint::Length(15), // Price
                Constraint::Length(15), // Liquidity
                Constraint::Length(12), // Lower Tick
                Constraint::Length(12), // Upper Tick
            ],
        )
        .header(headers) // Add headers to the table
        .block(
            Block::default()
                .borders(ratatui::widgets::Borders::ALL) // Add borders around the table
                .title("Pools") // Add a title to the table
        );

        // Render the table
        table.render(list_area, buf);
    }
}