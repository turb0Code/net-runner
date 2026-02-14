use crate::data_sources::sending::fetch_random_packet;
use crate::core::matrix_backend::{
    generate_path,
    handle_enter_key,
    handle_key_event,
    handle_mouse_event
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Paragraph}, prelude::*,
};
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant}};

pub enum AppExit {
    Quit,
    Reload
}

#[derive(Clone)]
pub struct MatrixState {
    pub bufor: [String; 7],
    pub cells: [[u8; 5]; 5],   // np. bajty w HEX
    pub hex_cells: [[String; 5]; 5], // reprezentacja tekstowa
    pub visited_cells: [[bool; 5]; 5],
    pub path1: Vec<[u8; 3]>,
    pub path2: Vec<[u8; 3]>,
    pub path3: Vec<[u8; 3]>,
    pub path1_tracking: i32,
    pub path2_tracking: i32,
    pub path3_tracking: i32,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub matrix_area: Option<Rect>,
    pub row_col: bool,
    pub active_row: u8,
    pub active_col: u8
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

fn format_path_line(label: &str, path: &Vec<[u8; 3]>, matrix: &[[u8; 5]; 5], path_state: i32) -> Line<'static>
{
    let mut spans = Vec::new();

    // nagłówek PATH X:
    spans.push(Span::styled(
        format!("{label}: "),
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ));

    for [r, c, flag] in path {
        let value = matrix[*r as usize][*c as usize];

        let color;
        if path_state == -1
        {
            color = Color::Red;
        }
        else if path_state == -2
        {
            color = Color::LightGreen;
            spans.push(Span::styled(
                format!("FINISHED"),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
            return Line::from(spans);
        }
        else
        {
            color = match flag {
                1 => Color::LightGreen,
                _ => Color::LightYellow,
            };
        }

        spans.push(Span::styled(
            format!("{:02X} ", value),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
    }

    Line::from(spans)
}

fn render_matrix(f: &mut Frame, area: Rect, state: &MatrixState)
{
    let mut lines = Vec::new();

    lines.push(Line::styled("\n", Style::default())); // Odstęp od tytułu,


    for y in 0..5 {
        let mut spans = Vec::new();

        for x in 0..5 {
            let value = &state.hex_cells[y][x];

            let mut style = Style::default().fg(Color::Cyan);

            if state.row_col && state.active_row as usize == y {
                style = style
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            if !state.row_col && state.active_col as usize == x {
                style = style
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            if state.cursor_x == x && state.cursor_y == y {
                style = style
                    .bg(Color::LightCyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            if state.visited_cells[y][x] {
                style = style.fg(Color::Gray).add_modifier(Modifier::BOLD);
            }

            spans.push(Span::styled(format!(" {} ", value), style));
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title(" DATA BREACH ").borders(Borders::ALL).add_modifier(Modifier::BOLD));

    f.render_widget(paragraph, area);

}

pub async fn main_interface() -> Result<AppExit, Box<dyn std::error::Error>>
{
    // Ustawienia terminala
        enable_raw_mode()?;
        execute!(stdout(), EnableMouseCapture)?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let raw_data = fetch_random_packet().await?;
        let matrix_cells = parse_data_to_matrix(&raw_data);
        let hex_matrix_cells = matrix_cells.map(|row| {
            row.map(|byte| format!("{:02X}", byte))
        });
        let bufor: [String; 7] = std::array::from_fn(|_| "XX".to_string());
        let mut counter = 0;

        let mut state = MatrixState { bufor: bufor, cells: matrix_cells, hex_cells: hex_matrix_cells, visited_cells: [[false; 5]; 5], path1: Vec::new(), path2: Vec::new(), path3: Vec::new(), path1_tracking: 0, path2_tracking: 0, path3_tracking: 0, cursor_x: 0, cursor_y: 0, matrix_area: None, row_col: true, active_col: 0, active_row: 0 };
        generate_path(&mut state);

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
                        if key.kind != KeyEventKind::Press {
                            continue;
                        }

                        if key.code == KeyCode::Char('q') {
                            return Ok(AppExit::Quit);
                        }

                        if key.code == KeyCode::Char(' ') || key.code == KeyCode::Enter
                        {

                            if state.path1_tracking < 0 && state.path2_tracking < 0 && state.path3_tracking < 0 || counter >= 7
                            {
                                return Ok(AppExit::Reload);
                            }
                            state.bufor[counter] = state.hex_cells[state.cursor_y][state.cursor_x].clone();
                            counter += 1;
                            handle_enter_key(&mut state);
                            continue;
                        }

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
                            Constraint::Length(3), // Bufor
                            Constraint::Length(5), // Task Data
                            Constraint::Length(9),   // Matrix
                            Constraint::Length(3), // Status bar
                        ])
                        .split(f.area());

                    // 1. Nagłówek
                    let header = Paragraph::new(" MATRIX DATA BREACH ")
                        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));
                    f.render_widget(header, chunks[0]);

                    let bufor_widget = Paragraph::new(" ".to_string() + &state.bufor.join(" "))
                        .style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD))
                        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::LightRed)).title(" BUFOR "));
                    f.render_widget(bufor_widget, chunks[1]);

                    let text = Text::from(vec![
                        format_path_line("PATH 1", &state.path1, &state.cells, state.path1_tracking),
                        format_path_line("PATH 2", &state.path2, &state.cells, state.path2_tracking),
                        format_path_line("PATH 3", &state.path3, &state.cells, state.path3_tracking),
                    ]);

                    let task_data = Paragraph::new(text)
                        .block(
                            Block::default()
                                .title(" DATA TO PICK ")
                                .borders(Borders::ALL)
                        )
                        .style(Style::default().fg(Color::LightYellow));

                    f.render_widget(task_data, chunks[2]);


                    if state.path1_tracking < 0 && state.path2_tracking < 0 && state.path3_tracking < 0 || counter >= 7
                    {
                        // Wszystkie ścieżki nieaktywne - koniec gry
                        let header = Paragraph::new(" HACKING ENDED \n\n 0xBD 0xED 0x21 \n .....0x5A..//. \n \\..0xFF...0xA1 \n\n PRESS 'ENTER' TO RELOAD ")
                            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
                        f.render_widget(header, chunks[3]);

                    }
                    else
                    {
                        state.matrix_area = Some(chunks[3]);
                        render_matrix(f, chunks[3], &state);
                    }

                    // 3. Status Bar
                    let status = Paragraph::new("Press 'Q' to abort hack...")
                        .style(Style::default().bg(Color::Red).fg(Color::White))
                        .block(Block::default().borders(Borders::ALL));
                    f.render_widget(status, chunks[4]);
                })?;
                last_tick = Instant::now();
            }
        }

        // Przywrócenie terminala
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(AppExit::Reload)
}