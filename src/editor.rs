use crate::{*, graphics::State, file::SaveFile};
use egui_macroquad::macroquad::{prelude::*, input};

pub fn mq_draw_editor(state: &mut State) {
    // Background Grid
    let col = state.theme.visuals.noninteractive().bg_stroke.color;
    let bg_col = Color {
        r: col.r() as f32 / 255.0,
        g: col.g() as f32 / 255.0,
        b: col.b() as f32 / 255.0,
        a: 1.0,
    };

    let spacing: f32 = state.zoom * LINE_DISTANCE;
    let width: f32 = 0.5;
    let offset: Vec2 = -Vec2::new(state.pos.x % spacing, state.pos.y % spacing);

    // X
    let mut x: f32 = offset.x;
    while x < screen_width() {
        x += spacing;
        draw_line(x, 0.0, x, screen_height(), width, bg_col);
    }

    // Y
    let mut y: f32 = offset.y;
    while y < screen_width() {
        y += spacing * 0.8;
        draw_line(0.0, y, screen_width(), y, width, bg_col);
    }

    // +/- Buttons
    let l_r_distance = 20.0;
    let centre_loc = Vec2::new(
        screen_width() / 32.0,
        screen_height() - (screen_height() / 32.0),
    );

    if input::is_mouse_button_down(MouseButton::Left)
        && ((centre_loc.x - l_r_distance) - input::mouse_position().0).abs() < 30.0
        && ((centre_loc.y) - input::mouse_position().1).abs() < 20.0
    {
        draw_circle(centre_loc.x - l_r_distance, centre_loc.y, 20.0, LIGHTGRAY);
        if !state.minus_button_debounce && state.zoom < MAX_ZOOM {
            state.zoom -= ZOOM_DELTA;
        }
        state.minus_button_debounce = true;
    } else {
        draw_circle(centre_loc.x - l_r_distance, centre_loc.y, 20.0, DARKGRAY);
        draw_rectangle(
            centre_loc.x - l_r_distance - 15.0,
            centre_loc.y - 1.0,
            30.0,
            2.0,
            GRAY,
        );
        state.minus_button_debounce = false;
    }

    if input::is_mouse_button_down(MouseButton::Left)
        && ((centre_loc.x + l_r_distance) - input::mouse_position().0).abs() < 30.0
        && ((centre_loc.y) - input::mouse_position().1).abs() < 20.0
    {
        draw_circle(centre_loc.x + l_r_distance, centre_loc.y, 20.0, LIGHTGRAY);
        if !state.plus_button_debounce && state.zoom > MIN_ZOOM {
            state.zoom += ZOOM_DELTA;
        }
        state.plus_button_debounce = true;
    } else {
        draw_circle(centre_loc.x + l_r_distance, centre_loc.y, 20.0, DARKGRAY);
        draw_rectangle(
            centre_loc.x + l_r_distance - 15.0,
            centre_loc.y - 1.0,
            30.0,
            2.0,
            GRAY,
        );
        draw_rectangle(
            centre_loc.x + l_r_distance - 1.0,
            centre_loc.y - 15.0,
            2.0,
            30.0,
            GRAY,
        );
        state.plus_button_debounce = false;
    }

    // Nodes
    if state.open_file.is_some() {
        // For input into draw functions
        let mut draw_state = state.clone();

        let data: &mut SaveFile = state.open_file.as_mut().unwrap();

        let zoom_percentage = state.zoom / MAX_ZOOM;

        let between_buffer: f32 = 50.0 * zoom_percentage * zoom_percentage;
        let element_height: f32 = 50.0 * zoom_percentage;
        let width: f32 = 300.0 * zoom_percentage;

        let nodes_clone = data.nodes.clone();

        let mut movement_selected = false;

        if !is_mouse_button_down(MouseButton::Left) {
            state.prev_movement_selected_node = None;
        }

        for node in &mut data.nodes {
            match node.clone().node_type {
                NodeType::Function => {
                    nodes::function_node::draw_function_node(
                        node,
                        element_height,
                        between_buffer,
                        zoom_percentage,
                        &mut draw_state,
                        width,
                        &mut movement_selected,
                        nodes_clone.clone(),
                    );
                }
                NodeType::Constant => {
                    nodes::constant_node::draw_constant_node(
                        node,
                        element_height,
                        between_buffer,
                        zoom_percentage,
                        &mut draw_state,
                        width,
                        &mut movement_selected,
                        nodes_clone.clone(),
                    );
                },
                NodeType::Variable => {
                    nodes::variable_node::draw_variable_node(
                        node,
                        element_height,
                        between_buffer,
                        zoom_percentage,
                        &mut draw_state,
                        width,
                        &mut movement_selected,
                        nodes_clone.clone(),
                    );
                },
                NodeType::Comment => {
                    nodes::comment_node::draw_comment_node(
                        &mut draw_state,
                        node,
                        zoom_percentage,
                        width,
                        &mut movement_selected,
                    );
                }
            }
        }
        state.last_mouse_click = draw_state.last_mouse_click;
        state.selected_node = draw_state.selected_node;
        state.prev_movement_selected_node = draw_state.prev_movement_selected_node;
        state.cur_comment = draw_state.cur_comment;
        state.cur_comment_open = draw_state.cur_comment_open;
        state.cur_const_data = draw_state.cur_const_data;
        state.cur_const_type = draw_state.cur_const_type;
        state.cur_var_data = draw_state.cur_var_data;
        state.cur_var_type = draw_state.cur_var_type;

        state.open_file = Some(data.clone());
    }
}