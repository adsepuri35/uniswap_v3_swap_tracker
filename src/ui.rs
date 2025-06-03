use std::collections::HashMap;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use ratatui::{
    buffer::Buffer,
    layout::{Rect, Constraint},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Table, Row, Cell},
    DefaultTerminal, Frame,
};
use anyhow::Result;
use tokio::sync::mpsc::Receiver;
use std::cmp::min;
use alloy::core::primitives::Address;

//file imports
use crate::pool_info::PoolInfo;
use crate::token_info::TokenInfo;
use crate::backend_update::BackendUpdate;

#[derive(Debug)]
pub struct TerminalUI   {
    exit: bool,
    total_swaps: usize,
    rx: Option<Receiver<BackendUpdate>>,
    scroll_offset: usize,
    selected_pool_index: usize,
    paused: bool,
    eth_swaps: usize,
    base_swaps: usize,
    arb_swaps: usize,
    show_prices: bool,
    pool_info_map: HashMap<Address, PoolInfo>,
    token_info_map: HashMap<Address, TokenInfo>,
    token_scroll_offset: usize,
    selected_token_index: usize,
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
            token_scroll_offset: 0,
            selected_token_index: 0,
        }
    }
}

impl TerminalUI {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            // check for events
            let handle_events = if let Some(_rx) = &mut self.rx {
                event::poll(std::time::Duration::from_millis(10))?
            } else {
                self.handle_events()?;
                false
            };
            
            if handle_events {
                self.handle_events()?;
            }
            
            // check for channel updates
            if let Some(rx) = &mut self.rx {
                match rx.try_recv() {
                    Ok(backend_update) => {
                        match backend_update {
                            BackendUpdate::PoolUpdated(pool) => {
                                self.pool_info_map.insert(pool.pool_address, pool);

                                self.total_swaps = self.pool_info_map.values().map(|p| p.swaps_tracked).sum();
                            }
                            BackendUpdate::TokenUpdated(token) => {
                                self.token_info_map.insert(token.address, token);
                            }
                            BackendUpdate::ChainStats { eth_swaps, base_swaps, arb_swaps } => {
                                self.eth_swaps = eth_swaps;
                                self.base_swaps = base_swaps;
                                self.arb_swaps = arb_swaps;
                            }
                        }
                    }
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                        // no data available
                    }
                    Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                        self.rx = None;
                    }
                }
            }
            
            // Draw UI
            if !self.paused {
                terminal.draw(|frame| self.draw(frame))?;
            }
            
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
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

            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_pool_index > 0 {
                    self.selected_pool_index -= 1;

                    if self.selected_pool_index < self.scroll_offset {
                        self.scroll_offset = self.selected_pool_index;
                    }
                }
            }

            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_pool_index < self.pool_info_map.len().saturating_sub(1) {
                    self.selected_pool_index += 1;

                    let max_visible = 10;
                    if self.selected_pool_index >= self.scroll_offset + max_visible {
                        self.scroll_offset = self.selected_pool_index + 1 - max_visible;
                    }
                }
            }

            KeyCode::PageUp => {
                let max_visible = 10;
                if self.scroll_offset > 0 {
                    self.scroll_offset = self.scroll_offset.saturating_sub(max_visible);
                    self.selected_pool_index = self.scroll_offset;
                }
            }

            KeyCode::PageDown => {
                let max_visible = 10;
                let max_scroll = self.pool_info_map.len().saturating_sub(max_visible);
                if self.scroll_offset < max_scroll {
                    self.scroll_offset = (self.scroll_offset + max_visible).min(max_scroll);
                    self.selected_pool_index = self.scroll_offset;
                }
            }

            KeyCode::Home => {
                self.scroll_offset = 0;
            }
            KeyCode::End => {
                self.scroll_offset = self.pool_info_map.len().saturating_sub(1);
            }

            KeyCode::Char('w') => {
                if self.selected_token_index > 0 {
                    self.selected_token_index -= 1;

                    if self.selected_token_index < self.token_scroll_offset {
                        self.token_scroll_offset = self.selected_token_index;
                    }
                }
            }

            KeyCode::Char('s') => {
                if self.selected_token_index < self.token_info_map.len().saturating_sub(1) {
                    self.selected_token_index += 1;

                    let max_visible = 10;
                    if self.selected_token_index >= self.token_scroll_offset + max_visible {
                        self.token_scroll_offset = self.selected_token_index + 1 - max_visible;
                    }
                }
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

    pub fn with_receiver(rx: Receiver<BackendUpdate>) -> Self {
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
            token_scroll_offset: 0,
            selected_token_index: 0,
        }
    }

}

impl Widget for &TerminalUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Up (Pool/Price) ".into(),
            "<↑/w>".blue().bold(),
            " Down (Pool/Price) ".into(),
            "<↓/s>".blue().bold(),
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

        // ui block
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_type(ratatui::widgets::BorderType::Rounded);

        block.clone().render(area, buf);

        let inner_area = block.inner(area);

        let header = format!(" Pools Tracked: {} | Swaps Tracked: {} | Tokens Tracked: {} | ETH Swaps: {} | BASE Swaps: {} | ARB Swaps: {} ", self.pool_info_map.len(), self.total_swaps, self.token_info_map.len(), self.eth_swaps, self.base_swaps, self.arb_swaps);
        let header_text = Text::from(header);

        Paragraph::new(header_text)
            .render(Rect::new(inner_area.x, inner_area.y, inner_area.width, 1), buf);

        let panes = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // pools block = 70%
                Constraint::Percentage(30), // prices block = 30%
            ])
            .split(Rect::new(
                inner_area.x,
                inner_area.y + 2,
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
            Cell::from("Chain"),
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
                    if color == ratatui::style::Color::White {
                        Cell::from(price_text).style(
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Yellow) // selected row = yellow
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
                    if volume.fract() == 0.0 {
                        format!("{:.0}", volume)
                    } else {
                        format!("{:.2}", volume)
                    }
                } else {
                    "0".to_string()
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
                Cell::from(
                    pool.networks
                        .iter()
                        .map(|network| format!("{:?}", network)) // Use Debug implementation to convert AlchemyNetwork to a string
                        .collect::<Vec<_>>()
                        .join(", ")
                ).style(style),
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
                Constraint::Length(21),
                Constraint::Length(12),
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

            let token_count = self.token_info_map.len();
            let max_visible = right_area.height as usize;
            let start_idx: usize = if token_count <= max_visible {
                0
            } else {
                let max_start = token_count.saturating_sub(max_visible);
                self.token_scroll_offset.min(max_start)
            };

            let visible_tokens: Vec<_> = self
                .token_info_map
                .values()
                .skip(start_idx)
                .take(max_visible)
                .collect();

            let rows: Vec<Row> = visible_tokens.iter().enumerate().map(|(i, token_info)| {
                let is_selected = start_idx + i == self.selected_token_index;

                let price = token_info.value.clone().unwrap_or("Unknown".to_string());
                let price_display = if price == "Unknown" {
                    price
                } else {
                    format!("${}", price)
                };

                let color = token_info.get_price_change_color();

                let style = if is_selected {
                    if color == ratatui::style::Color::White {
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Yellow)
                            .add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        ratatui::style::Style::default()
                            .fg(color)
                            .add_modifier(ratatui::style::Modifier::BOLD)
                    }
                } else {
                    ratatui::style::Style::default().fg(color)
                };

                Row::new(vec![
                    Cell::from(token_info.symbol.clone()).style(style),
                    Cell::from(price_display).style(style),
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
            // render swaps for selected pool
            if let Some(selected_pool) = sorted_pools.get(self.selected_pool_index) {
                let key = selected_pool.pool_address.clone();
                if let Some(pool) = self.pool_info_map.get(&key) {
                    let swap_store = &pool.swap_store; // get swap events

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

                    // no swaps -> empty row
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
                // no pools -> empty swaps table
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
