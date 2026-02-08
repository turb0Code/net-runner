use ratatui::{
    Terminal, backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style, Stylize}, widgets::{Block, Borders, List, ListItem, Paragraph}, prelude::*,
};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::{self, stdout}, sync::{Arc, Mutex}, thread, time::{Duration, Instant}};
use crate::data_sources::sending::fetch_random_packet;
use rand::Rng;

#[derive(Clone)]
pub struct MatrixState {
    pub cells: [[u8; 5]; 5],   // np. bajty w HEX
    pub hex_cells: [[String; 5]; 5], // reprezentacja tekstowa
    pub path1: Vec<[u8; 2]>,
    pub path2: Vec<[u8; 2]>,
    pub path3: Vec<[u8; 2]>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub matrix_area: Option<Rect>
}

fn parse_data_to_matrix(data: &[u8]) -> [[u8; 5]; 5]
{
    let mut matrix = [[0u8; 5]; 5];
    for (i, byte) in data.iter().take(25).enumerate() {
        let x = i % 5;
        let y = i / 5;
        matrix[y][x] = *byte;
    }
    matrix
}

fn generate_path(matrix: &[[u8; 5]; 5], length: usize, state: &mut MatrixState)
{
    let mut path1: Vec<[u8;2]> = Vec::new();
    let mut path2: Vec<[u8;2]> = Vec::new();
    let mut path3: Vec<[u8;2]> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    let mut rng = rand::thread_rng();
    for i in 0..length {
        let random = rng.gen_range(0..4);
        if i % 2 == 0
        {
            path3.push([y,random]);
            x = random;
        }
        else {
            path3.push([random, x]);
            y = random;
        }
    }
    state.path3 = path3;



}



fn render_matrix(f: &mut Frame, area: Rect, state: &MatrixState)
{
    let mut lines = Vec::new();

    for y in 0..5 {
        let mut spans = Vec::new();

        for x in 0..5 {
            let value = &state.hex_cells[y][x];
            let style = if state.cursor_x == x && state.cursor_y == y {
                Style::default()
                    .bg(Color::LightYellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };

            spans.push(Span::styled(format!(" {} ", value), style));
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title(" DATA BREACH ").borders(Borders::ALL).add_modifier(Modifier::BOLD));

    f.render_widget(paragraph, area);
}

fn handle_key_event(key: KeyCode, state: &mut MatrixState)
{
    match key {
        KeyCode::Up => {
            if state.cursor_y > 0 {
                state.cursor_y -= 1;
            }
        }
        KeyCode::Down => {
            if state.cursor_y < 4 {
                state.cursor_y += 1;
            }
        }
        KeyCode::Left => {
            if state.cursor_x > 0 {
                state.cursor_x -= 1;
            }
        }
        KeyCode::Right => {
            if state.cursor_x < 4 {
                state.cursor_x += 1;
            }
        }
        _ => {}
    }
}

fn handle_mouse_event(
    mouse: crossterm::event::MouseEvent,
    matrix_area: Rect,
    state: &mut MatrixState,
) {
    if let MouseEventKind::Down(_) = mouse.kind {
        let x = mouse.column;
        let y = mouse.row;

        if matrix_area.contains(Position { x, y }) {
            let cell_width = 3; // " XX "
            let rel_x = x - matrix_area.x;
            let rel_y = y - matrix_area.y - 1; // title offset

            let cx = (rel_x / cell_width) as usize;
            let cy = rel_y as usize;

            if cx < 5 && cy < 5 {
                state.cursor_x = cx;
                state.cursor_y = cy;
            }
        }
    }
}

pub async fn main_interface() -> Result<(), Box<dyn std::error::Error>>
{
    // Ustawienia terminala
        enable_raw_mode()?;
        execute!(stdout(), EnableMouseCapture)?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Współdzielona lista pakietów (hakerskie logi)
        let packets = Arc::new(Mutex::new(vec![String::from("System initialized...")]));
        let packets_clone = Arc::clone(&packets);

        let raw_data = fetch_random_packet().await?;
        let matrix_cells = parse_data_to_matrix(&raw_data);
        let hex_matrix_cells = matrix_cells.map(|row| {
            row.map(|byte| format!("{:02X}", byte))
        });

        let mut state = MatrixState { cells: matrix_cells, hex_cells: hex_matrix_cells, cursor_x: 0, cursor_y: 0, matrix_area: None };

        let tick_rate = Duration::from_millis(16); // ~60 FPS
        let mut last_tick = Instant::now();

        // PĘTLA UI
        loop {

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            // Obsługa klawiszy
            if event::poll(timeout)? {
                match event::read()?
                {
                    Event::Key(key) => 
                    {
                        if key.code == KeyCode::Char('q') { break; }
                        handle_key_event(key.code, &mut state);
                    }
                    Event::Mouse(mouse) =>
                    {
                        if let Some(area) = state.matrix_area {
                            handle_mouse_event(mouse, area, &mut state);
                        }
                    }
                    Event::Resize(_, _) => {}
                    Event::FocusGained | Event::FocusLost | Event::Paste(_) => todo!()
                }
              
            }

            if last_tick.elapsed() >= tick_rate
            {
                terminal.draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3), // Header
                            Constraint::Length(5),
                            Constraint::Min(10),   // Main log
                            Constraint::Length(3), // Status bar
                        ])
                        .split(f.area());

                    // 1. Nagłówek
                    let header = Paragraph::new(" MATRIX DATA BREACH ")
                        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));
                    f.render_widget(header, chunks[0]);

                    let task_data = Paragraph::new(" 3F 5D\n 4C BD A2\n 5C 4C 81 A2 1B")
                        .block(Block::default().title(" DATA TO PICK ")
                        .add_modifier(Modifier::BOLD)
                        .borders(Borders::ALL))
                        .style(Style::default().fg(Color::LightYellow));
                    f.render_widget(task_data, chunks[1]);

                    state.matrix_area = Some(chunks[2]);
                    render_matrix(f, chunks[2], &state);
                        
                    // 3. Status Bar
                    let status = Paragraph::new("Press 'Q' to abort hack...")
                        .style(Style::default().bg(Color::Red).fg(Color::White))
                        .block(Block::default().borders(Borders::ALL));
                    f.render_widget(status, chunks[3]);
                })?;
                last_tick = Instant::now();
            }
        }

        // Przywrócenie terminala
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
}