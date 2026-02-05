use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Color, Modifier, Style},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, sync::{Arc, Mutex}};

pub fn main_interface() -> Result<(), Box<dyn std::error::Error>>
{
    // Ustawienia terminala
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Współdzielona lista pakietów (hakerskie logi)
        let packets = Arc::new(Mutex::new(vec![String::from("System initialized...")]));
        let packets_clone = Arc::clone(&packets);

        // WĄTEK "HAKOWANIA" (Symulacja lub Twoje pnet)
        thread::spawn(move || {
            let mut count = 0;
            loop {
                {
                    let mut p = packets_clone.lock().unwrap();
                    p.push(format!("[DECRYPTED] Packet #{}: SEQ={} ACK={}", count, 1000 + count, 2000 + count));
                    if p.len() > 20 { p.remove(0); } // Keep it clean
                }
                count += 1;
                thread::sleep(std::time::Duration::from_millis(500));
            }
        });

        // PĘTLA UI
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Header
                        Constraint::Min(10),   // Main log
                        Constraint::Length(3), // Status bar
                    ])
                    .split(f.area());

                // 1. Nagłówek
                let header = Paragraph::new(" MATRIX PACKET SNIFFER v1.0 ")
                    .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));
                f.render_widget(header, chunks[0]);

                // 2. Lista pakietów
                let p_list = packets.lock().unwrap();
                let items: Vec<ListItem> = p_list.iter().map(|s| ListItem::new(s.as_str())).collect();
                let list = List::new(items)
                    .block(Block::default().title(" LIVE DATA STREAM ").borders(Borders::ALL))
                    .style(Style::default().fg(Color::Cyan));
                f.render_widget(list, chunks[1]);

                // 3. Status Bar
                let status = Paragraph::new("Press 'Q' to abort hack...")
                    .style(Style::default().bg(Color::Red).fg(Color::White))
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(status, chunks[2]);
            })?;

            // Obsługa klawisza Q
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') { break; }
                }
            }
        }

        // Przywrócenie terminala
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
}