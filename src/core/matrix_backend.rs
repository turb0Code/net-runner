use crate::core::matrix_mode::MatrixState;

use rand::Rng;
use crossterm::event::{
    KeyCode,
    MouseEventKind
};
use ratatui::prelude::*;

pub fn generate_path(state: &mut MatrixState)
{
    let mut visited = [[false; 5]; 5];
    let mut path1: Vec<[u8;3]> = Vec::new();
    let mut path2: Vec<[u8;3]> = Vec::new();
    let mut path3: Vec<[u8;3]> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut counter = 0;

    let mut rng = rand::thread_rng();
    while path3.len() < 5 {
        let random = rng.gen_range(0..5);
        if counter % 2 == 0
        {
            if visited[y as usize][random as usize] { continue; }
            visited[y as usize][random as usize] = true;
            path3.push([y,random, 0]);
            x = random;
        }
        else {
            if visited[random as usize][x as usize] { continue; }
            visited[random as usize][x as usize] = true;
            path3.push([random, x, 0]);
            y = random;
        }
        counter += 1;
    }

    let mut pick = rng.gen_range(0..3);
    let point1 = path3[pick];
    let point3 = path3[pick+2];
    let point2;

    let random_pick = rng.gen_range(0..2);
    if random_pick == 0
    {
        point2 = [point1[0], point3[1], 0];
    }
    else
    {
        point2 = [point3[0], point1[1], 0];
    }
    path2.push(point1);
    path2.push(point2);
    path2.push(point3);

    pick = rng.gen_range(0..5);
    x = pick as u8;
    path1.push([0, x, 0]);
    pick = rng.gen_range(1..5);
    path1.push([pick as u8, x, 0]);


    state.path1 = path1;
    state.path2 = path2;
    state.path3 = path3;
}

pub fn handle_enter_key(state: &mut MatrixState)
{
    state.row_col = !state.row_col;
    state.active_row = state.cursor_y as u8;
    state.active_col = state.cursor_x as u8;

    if state.path1_tracking >= 0 && state.visited_cells[state.cursor_y][state.cursor_x] == false
    {
        if state.hex_cells[state.path1[state.path1_tracking as usize][0] as usize][state.path1[state.path1_tracking as usize][1] as usize] == state.hex_cells[state.cursor_y][state.cursor_x]
        {
            state.path1[state.path1_tracking as usize][2] = 1;
            state.path1_tracking += 1;
            if state.path1_tracking >= state.path1.len() as i32
            {
                state.path1_tracking = -2;
            }
        }
        else
        {
            if state.path1_tracking > 0
            {
                state.path1_tracking = -1;
            }
        }
    }

    if state.path2_tracking >= 0 && state.visited_cells[state.cursor_y][state.cursor_x] == false
    {
        if state.hex_cells[state.path2[state.path2_tracking as usize][0] as usize][state.path2[state.path2_tracking as usize][1] as usize] == state.hex_cells[state.cursor_y][state.cursor_x]
        {
            state.path2[state.path2_tracking as usize][2] = 1;
            state.path2_tracking += 1;
            if state.path2_tracking >= state.path2.len() as i32
            {
                state.path2_tracking = -2;
            }
        }
        else
        {
            if state.path2_tracking > 0
            {
                state.path2_tracking = -1;
            }
        }
    }

    if state.path3_tracking >= 0 && state.visited_cells[state.cursor_y][state.cursor_x] == false
    {
        if state.hex_cells[state.path3[state.path3_tracking as usize][0] as usize][state.path3[state.path3_tracking as usize][1] as usize] == state.hex_cells[state.cursor_y][state.cursor_x]
        {
            state.path3[state.path3_tracking as usize][2] = 1;
            state.path3_tracking += 1;
            if state.path3_tracking >= state.path3.len() as i32
            {
                state.path3_tracking = -2;
            }
        }
        else
        {
            if state.path3_tracking > 0
            {
                state.path3_tracking = -1;
            }
        }
    }

    state.visited_cells[state.cursor_y][state.cursor_x] = true;
}

pub fn handle_key_event(key: KeyCode, state: &mut MatrixState)
{
    match key {
        KeyCode::Up => {
            if state.cursor_y > 0 && state.row_col == false {
                state.cursor_y -= 1;
            }
        }
        KeyCode::Down => {
            if state.cursor_y < 4 && state.row_col == false {
                state.cursor_y += 1;
            }
        }
        KeyCode::Left => {
            if state.cursor_x > 0 && state.row_col == true {

                state.cursor_x -= 1;
            }
        }
        KeyCode::Right => {
            if state.cursor_x < 4 && state.row_col == true {
                state.cursor_x += 1;
            }
        }
        _ => {}
    }
}

pub fn handle_mouse_event(mouse: crossterm::event::MouseEvent, matrix_area: Rect, state: &mut MatrixState)
{
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