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

//file imports
use crate::poollnfo::PoolInfo;

#[derive(Debug)]
pub struct TerminalUI {
    exit: bool,
    pools: Vec<PoolInfo>,
    total_swaps: usize,
    rx: Option<Receiver<Vec<PoolInfo>>>,
}

impl Default for TerminalUI {
    fn default() -> Self {
        Self {
            exit: false,
            pools: Vec::new(),
            total_swaps: 0,
            rx: None,
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
                        println!("Channel disconnected");
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
        }
    }

}

impl Widget for &TerminalUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Uniswap v3 Swap Terminal ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let header = Line::from(vec![
            "Pool".bold().yellow(),
            " | ".into(),
            "Swaps Tracked".bold().yellow(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);


        // lines for each pool
        let mut content_lines = vec![header];


        // display total stats
        content_lines.push(Line::from(vec![
            "Total Swaps: ".into(),
            self.total_swaps.to_string().bold(),
        ]));

        // add lines for each pool
        for pool in &self.pools {
            content_lines.push(Line::from(vec![
                pool.get_pool_name().into(),
                " | ".into(),
                pool.get_swap_count().to_string().green(),
            ]));
        }

        // if no pools -> show message
        if self.pools.is_empty() {
            content_lines.push(Line::from("No pools detected yet. Waiting for events...".italic()));
        }

        let content_text = Text::from(content_lines);

        Paragraph::new(content_text)
            .block(block)
            .render(area, buf);
    }
}