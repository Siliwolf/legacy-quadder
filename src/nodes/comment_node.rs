use egui_macroquad::macroquad::prelude::*;

use crate::{graphics::State, Node};

pub fn draw_comment_node(
    state: &mut State,
    node: &mut Node,
    zoom_percentage: f32,
    width: f32,
    movement_selected: &mut bool,
) {
    // Prevent node from disappearing with no characters
    if state.cur_comment.is_empty() {
        state.cur_comment = " ".to_owned()
    }

    if state.cur_comment.len() > 1 && state.cur_comment.chars().nth(0).unwrap() == ' ' {
        state.cur_comment.remove(0);
    }

    // Fetch data from dialog
    if node.selected {
        node.name = state.cur_comment.clone();
    }

    let top_height: f32 = 150.0 * zoom_percentage;

    let x = node.position[0] + -state.pos.x;
    let y = node.position[1] + -state.pos.y;

    let node_name = node.name.clone();
    let mut node_name_copy = node_name.clone();

    let mut _top_text_fits = false;
    let mut top_text_fits_first = true;
    while !_top_text_fits {
        let d = measure_text(
            &node_name_copy,
            Some(state.font_mq),
            (top_height * 0.5) as u16,
            1.0,
        );
        if d.width < width {
            _top_text_fits = true;
            break;
        }

        top_text_fits_first = false;
        node_name_copy.pop();
    }

    if !top_text_fits_first {
        node_name_copy.pop();
        node_name_copy.pop();
        node_name_copy.pop();
        node_name_copy.pop();
        node_name_copy.push_str("...");
    }

    let top_text_params = TextParams {
        font: state.font_mq,
        font_size: (top_height * 0.5) as u16,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: WHITE,
    };

    let mut height: f32 = measure_text(
        &node_name_copy,
        Some(state.font_mq),
        (top_height * 0.5) as u16,
        1.0,
    )
    .height;
    height *= zoom_percentage;
    height += top_height;
    height *= 2.0;

    // Main body
    draw_rectangle(
        x - (width / 2.0),
        y - (height / 2.0),
        width,
        height,
        Color {
            r: 0.35,
            g: 0.35,
            b: 0.252,
            a: 1.0,
        },
    );

    // Top Section
    draw_rectangle(
        x - (width / 2.0),
        y - (height / 2.0),
        width,
        top_height,
        Color {
            r: 0.2,
            g: 0.2,
            b: 0.22,
            a: 1.0,
        },
    );

    // "Comment" Title
    draw_text_ex(
        &"Comment".to_owned(),
        x - (width / 2.0),
        y - (height / 2.0 - (top_height * 0.8)),
        top_text_params,
    );

    let lmc = Vec2::new(mouse_position().0, mouse_position().1);

    if (lmc.x > (x - (width / 2.0)) && lmc.x < x - (width / 2.0) + width)
        && (lmc.y > y - (height / 2.0) && lmc.y < y - (height / 2.0) + top_height)
    {
        if is_mouse_button_down(MouseButton::Left)
            && !*movement_selected
            && (state.prev_movement_selected_node.is_none()
                || state.prev_movement_selected_node.as_ref().unwrap().id == node.id)
        {
            state.last_mouse_click = Vec2::new(mouse_position().0, mouse_position().1);
            let offset: Vec2 = Vec2::new(0.0, (height / 2.0) - (top_height / 2.0));
            node.position = [
                mouse_position().0 + offset.x + state.pos.x,
                mouse_position().1 + offset.y + state.pos.y,
            ];
            *movement_selected = true;
            state.prev_movement_selected_node = Some(node.clone());
        } else {
            state.prev_movement_selected_node = None;
        }
    }

    draw_text_ex(
        &node_name_copy,
        x - (width / 2.0),
        y + (top_height * 0.5),
        top_text_params,
    );

    // Selection
    let lmc = state.last_mouse_click.clone();

    if (lmc.x > (x - (width / 2.0)) && lmc.x < x - (width / 2.0) + width)
        && (lmc.y > y - (height / 2.0) && lmc.y < y - (height / 2.0) + height)
    {
        state.selected_node = Some(node.clone());
        draw_rectangle_lines(x - width / 2.0, y - height / 2.0, width, height, 1.0, WHITE);

        // Open Dialog
        if is_mouse_button_down(MouseButton::Left)
            && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
        {
            node.selected = true;
            state.cur_comment_open = true;
            state.cur_comment = node.name.clone();
        }
    }
}
