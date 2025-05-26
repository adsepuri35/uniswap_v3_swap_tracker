use std::io;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, terminal};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
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
        
        // Calculate how many pools we can display at once
        let visible_height = inner_area.height as usize;
        
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
            inner_area.height.saturating_sub(2)
        );
        
        // Determine visible range of pools based on scroll position
        let pool_count = self.pools.len();
        let max_visible = list_area.height as usize;
        
        // Calculate the start index based on scroll offset
        let start_idx: usize = if pool_count <= max_visible {
            0 // Show from beginning if all fit
        } else {
            // Otherwise, use scroll offset to determine starting point
            // Make sure we can always see a full page
            let max_start = pool_count.saturating_sub(max_visible);
            self.scroll_offset.min(max_start)
        };
        
        // Create a visible slice of pools
        let visible_pools = &self.pools[start_idx..min(start_idx + max_visible, pool_count)];
        
        // Render each visible pool
        for (i, pool) in visible_pools.iter().enumerate() {
            let y_pos = list_area.y + i as u16;
            
            // Skip if we're past the bottom of the visible area
            if y_pos >= list_area.y + list_area.height {
                break;
            }

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
            
            let line = format!(
                "{:3}. {:30} | {:5} swaps | price: {:20}", 
                start_idx + i + 1,
                pool.get_pool_name(),
                pool.get_swap_count(),
                price_display,
            );
            
            // Render this pool line
            Paragraph::new(Text::from(line))
                .render(Rect::new(list_area.x, y_pos, list_area.width, 1), buf);
        }
        
        // Render scroll indicators if needed
        if pool_count > max_visible {
            let scroll_info = format!("[{}/{}]", start_idx + 1, pool_count);
            Paragraph::new(Text::from(scroll_info.clone()))
                .render(
                    Rect::new(
                        inner_area.x + inner_area.width - scroll_info.len() as u16 - 2,
                        inner_area.y + inner_area.height - 1,
                        scroll_info.len() as u16,
                        1
                    ),
                    buf
                );
        }
    }
}