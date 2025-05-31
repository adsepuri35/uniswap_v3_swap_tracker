use std::hash::Hash;
use std::{io, ops::Add};
use std::collections::HashMap;
use amms::amms::Token;
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
use std::cmp::min;
use alloy::core::primitives::Address;

//file imports
use crate::poollnfo::PoolInfo;
use crate::tokenInfo::TokenInfo;

#[derive(Debug)]
pub struct TerminalUI   {
    exit: bool,
    total_swaps: usize,
    rx: Option<Receiver<(HashMap<(String, Address), PoolInfo>, usize, usize, usize, HashMap<Address, TokenInfo>)>>,
    scroll_offset: usize,
    selected_pool_index: usize,
    paused: bool,
    eth_swaps: usize,
    base_swaps: usize,
    arb_swaps: usize,
    show_prices: bool,
    pool_info_map: HashMap<(String, Address), PoolInfo>,
    token_info_map: HashMap<Address, TokenInfo>,
}

impl Default for TerminalUI {
    fn default() -> Self {
        Self {
            exit: false,
            total_swaps: 0,
            rx: None,
            scroll_offset: 0,
            selected_pool_index: 0,
            paused: false,
            eth_swaps: 0,
            base_swaps: 0,
            arb_swaps: 0,
            show_prices: false,
            pool_info_map: HashMap::new(),
            token_info_map: HashMap::new(),
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
                    Ok((pool_info_map, eth_swaps, base_swaps, arb_swaps, token_info_map)) => {
                        // Update the UI with new pool data
                        self.pool_info_map = pool_info_map;
                        self.total_swaps = self.pool_info_map.values().map(|p| p.swaps_tracked).sum();
                        self.eth_swaps = eth_swaps;
                        self.base_swaps = base_swaps;
                        self.arb_swaps = arb_swaps;
                        self.token_info_map = token_info_map;
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
            if !self.paused {
                terminal.draw(|frame| self.draw(frame))?;
            }
            
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
            // quit
            KeyCode::Char('q') => self.exit(),

            // pause
            KeyCode::Char('p') => {
                self.paused = !self.paused;
            }

            // toggle prices
            KeyCode::Char('t') => {
                self.show_prices = !self.show_prices
            }

            // Add scroll controls
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_pool_index > 0 {
                    self.selected_pool_index -= 1;

                    // Adjust scroll offset if the selected pool is above the visible range
                    if self.selected_pool_index < self.scroll_offset {
                        self.scroll_offset = self.selected_pool_index;
                    }
                }
            }

            // Move selection down
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_pool_index < self.pool_info_map.len().saturating_sub(1) {
                    self.selected_pool_index += 1;

                    // Adjust scroll offset if the selected pool is below the visible range
                    let max_visible = 10; // Number of rows visible at a time
                    if self.selected_pool_index >= self.scroll_offset + max_visible {
                        self.scroll_offset = self.selected_pool_index + 1 - max_visible;
                    }
                }
            }

            // Scroll a page up
            KeyCode::PageUp => {
                let max_visible = 10; // Number of rows visible at a time
                if self.scroll_offset > 0 {
                    self.scroll_offset = self.scroll_offset.saturating_sub(max_visible);
                    self.selected_pool_index = self.scroll_offset;
                }
            }

            // Scroll a page down
            KeyCode::PageDown => {
                let max_visible = 10; // Number of rows visible at a time
                let max_scroll = self.pool_info_map.len().saturating_sub(max_visible);
                if self.scroll_offset < max_scroll {
                    self.scroll_offset = (self.scroll_offset + max_visible).min(max_scroll);
                    self.selected_pool_index = self.scroll_offset;
                }
            }

            KeyCode::Home => {
                // Scroll to the top
                self.scroll_offset = 0;
            }
            KeyCode::End => {
                // Scroll to the bottom
                self.scroll_offset = self.pool_info_map.len().saturating_sub(1);
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

    pub fn with_receiver(rx: Receiver<(HashMap<(String, Address), PoolInfo>, usize, usize, usize, HashMap<Address, TokenInfo>)>) -> Self {
        Self {
            exit: false,
            total_swaps: 0,
            rx: Some(rx),
            scroll_offset: 0,
            selected_pool_index: 0,
            paused: false,
            eth_swaps: 0,
            base_swaps: 0,
            arb_swaps: 0,
            show_prices: false,
            pool_info_map: HashMap::new(),
            token_info_map: HashMap::new(),
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
            " Show Prices ".into(),
            "<T>".blue().bold(),
            " Pause ".into(),
            "<P>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let title = Line::from(" Uniswap v3 Swap Tracker (ARB, BASE, ETH) ".magenta().bold());

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
        let header = format!(" Pools Tracked: {} | Swaps Tracked: {} | Tokens Tracked: {} | ETH Swaps: {} | BASE Swaps: {} | ARB Swaps: {} ", self.pool_info_map.len(), self.total_swaps, self.token_info_map.len(), self.eth_swaps, self.base_swaps, self.arb_swaps);
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

        let mut sorted_pools: Vec<PoolInfo> = self.pool_info_map.values().cloned().collect();
        sorted_pools.sort_by(|a, b| b.swaps_tracked.cmp(&a.swaps_tracked));

        let pool_count = sorted_pools.len();
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

            let is_selected = start_idx + i == self.selected_pool_index;

            let price_display = {
                let price = pool.current_price;
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

                let color = pool.get_price_change_color();
                
                if is_selected {
                    if (color == ratatui::style::Color::White) {
                        Cell::from(price_text).style(
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Yellow) // Use yellow for selected rows
                            .add_modifier(ratatui::style::Modifier::BOLD),
                        )
                    } else {
                        Cell::from(price_text).style(
                            ratatui::style::Style::default()
                                .fg(color)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        )
                    }
                    
                } else {
                    // Use the price change color for non-selected rows
                    Cell::from(price_text).style(ratatui::style::Style::default().fg(color))
                }
            };

            let liquidity_display = {
                let liquidity = pool.liquidity;
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

            let (lower_tick, upper_tick) = pool.tick_range;

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

            
            let style = if is_selected {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD)
            } else {
                ratatui::style::Style::default()
            };

            Row::new(vec![
                Cell::from(format!("{}", start_idx + i +  1)).style(style), // Index
                Cell::from(pool.pool_name.to_string()).style(style), // Pool Name
                Cell::from("v3").style(style),
                Cell::from(format!("{:.2}%", pool.get_fee_percent())).style(style), // Fee
                Cell::from(format!("{}", pool.swaps_tracked)).style(style), // Swaps
                price_display, // Price
                Cell::from(liquidity_display).style(style), // Liquidity
                Cell::from(format!("{}", lower_tick)).style(style), // Lower Tick
                Cell::from(format!("{}", upper_tick)).style(style), // Upper Tick
                Cell::from(format!("{:.2}%", pool.current_apr)).style(style), // APR
                Cell::from(format_volume(pool.volume)).style(style), // Volume
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



        if self.show_prices {

            let headers = Row::new(vec![
                Cell::from("Token"),
                Cell::from("Price (USD)"),
            ])
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD));

            let rows: Vec<Row> = self.token_info_map.values().map(|token_info| {
                let price = token_info.value.clone().unwrap_or("Unknown".to_string());
                let price_display = if price == "Unknown" {
                    price // No `$` sign for unknown prices
                } else {
                    format!("${}", price) // Add `$` sign for known prices
                };
                Row::new(vec![
                    Cell::from(token_info.symbol.clone()),
                    Cell::from(price_display),
                ])
            }).collect();

            let prices_table = Table::new(
                rows,
                vec![
                    Constraint::Length(30),
                    Constraint::Length(30),
                ],
            )
            .header(headers)
            .block(
                Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title(" Prices "),
            );

            prices_table.render(right_area, buf);

        } else {
            // Render swaps for the selected pool
            if let Some(selected_pool) = sorted_pools.get(self.selected_pool_index) {
                // Use the pool address to find the correct index in the original list
                let key = (selected_pool.network.clone(), selected_pool.pool_address.clone());
                if let Some(pool) = self.pool_info_map.get(&key) {
                    let swap_store = &pool.swap_store; // Retrieve the swap events

                    let headers = Row::new(vec![
                        Cell::from("Timestamp"),
                        Cell::from(format!("Amount of {}", pool.get_token0_symbol())),
                        Cell::from(format!("Amount of {}", pool.get_token1_symbol())),
                    ])
                    .style(
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Cyan)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    );

                    // If there are no swaps, display an empty row
                    let rows: Vec<Row> = if swap_store.is_empty() {
                        vec![Row::new(vec![
                            Cell::from("No swaps available"),
                            Cell::from(""),
                            Cell::from(""),
                        ])]
                    } else {
                        swap_store
                            .iter()
                            .rev()
                            .map(|(amount0, amount1, timestamp)| {
                                Row::new(vec![
                                    Cell::from(format!("{}", timestamp)), // Timestamp
                                    Cell::from(format!("{}", amount0)),  // Amount0
                                    Cell::from(format!("{}", amount1)),  // Amount1
                                ])
                            })
                            .collect()
                    };

                    let swaps_table = Table::new(
                        rows,
                        vec![
                            Constraint::Length(25),
                            Constraint::Length(20),
                            Constraint::Length(20),
                        ],
                    )
                    .header(headers)
                    .block(
                        Block::default()
                            .borders(ratatui::widgets::Borders::ALL)
                            .title(format!(" Swaps for Pool: {} ", pool.pool_name)),
                    );

                    swaps_table.render(right_area, buf);
                }
                
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
}
