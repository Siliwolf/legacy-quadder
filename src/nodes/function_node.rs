use egui_macroquad::macroquad::{input, prelude::*};

use crate::{graphics::State, *};

pub fn draw_function_node(
    node: &mut Node,
    element_height: f32,
    between_buffer: f32,
    zoom_percentage: f32,
    state: &mut State,
    width: f32,
    movement_selected: &mut bool,
    nodes_clone: Vec<Node>,
) {
    let node_copy = node.clone();

    let mut height: f32 = (node.input_connectors.len() as f32 * element_height)
        + (node.output_connectors.len() as f32 * element_height)
        + ((node.position.len() + node.input_connectors.len() + node.output_connectors.len())
            as f32
            * between_buffer);
    height *= zoom_percentage;
    let top_height: f32 = 150.0 * zoom_percentage;
    height += top_height;
    height *= 2.0;

    let x = node.position[0] + -state.pos.x;
    let y = node.position[1] + -state.pos.y;

    // Main body
    draw_rectangle(
        x - (width / 2.0),
        y - (height / 2.0),
        width,
        height,
        Color {
            r: 0.25,
            g: 0.25,
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

    // Node Movement
    let lmc = Vec2::new(mouse_position().0, mouse_position().1);

    if (lmc.x > (x - (width / 2.0)) && lmc.x < x - (width / 2.0) + width)
        && (lmc.y > y - (height / 2.0) && lmc.y < y - (height / 2.0) + top_height)
    {
        if input::is_mouse_button_down(MouseButton::Left)
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

    // Name Culling
    let mut node_name = node.name.clone();

    let top_text_params = TextParams {
        font: state.font_mq,
        font_size: (top_height * 0.5) as u16,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: WHITE,
    };

    let mut _top_text_fits = false;
    let mut top_text_fits_first = true;
    while !_top_text_fits {
        let d = measure_text(
            &node_name,
            Some(state.font_mq),
            (top_height * 0.5) as u16,
            1.0,
        );
        if d.width < width {
            _top_text_fits = true;
            break;
        }

        top_text_fits_first = false;
        node_name.pop();
    }

    if !top_text_fits_first {
        node_name.pop();
        node_name.pop();
        node_name.pop();
        node_name.push_str("...");
    }

    draw_text_ex(
        &node_name,
        x - (width / 2.0),
        y - (height / 2.0 - (top_height * 0.8)),
        top_text_params,
    );

    // Selection
    let lmc = state.last_mouse_click.clone();

    if (lmc.x > (x - (width / 2.0)) && lmc.x < x - (width / 2.0) + width)
        && (lmc.y > y - (height / 2.0) && lmc.y < y - (height / 2.0) + height)
    {
        state.selected_node = Some(node.clone());
        draw_rectangle_lines(x - width / 2.0, y - height / 2.0, width, height, 1.0, WHITE);
    }

    // Connectors
    let left_side = x - (width / 2.0);
    let right_side = x + (width / 2.0);

    let mut idx: u32 = 0;
    for c in &mut node.input_connectors {
        let c_name = &mut ("    ".to_owned() + &c.name);

        let circle_color = match c.connector_type {
            PropertyType::Bool => GREEN,
            PropertyType::Number => BLUE,
            PropertyType::String => RED,
            PropertyType::List => ORANGE,
            PropertyType::File => YELLOW,
            PropertyType::Flow => GRAY,
            PropertyType::Custom(_) => PURPLE,
        };

        let mut _top_text_fits = false;
        let mut top_text_fits_first = true;
        while !_top_text_fits {
            let d = measure_text(&c_name, Some(state.font_mq), element_height as u16, 1.0);
            if d.width < width {
                _top_text_fits = true;
                break;
            }

            top_text_fits_first = false;
            c_name.pop();
        }

        if !top_text_fits_first {
            c_name.pop();
            c_name.pop();
            c_name.pop();
            c_name.push_str("...");
        }

        draw_text_ex(
            &c_name,
            left_side,
            get_connector_height(between_buffer, element_height, idx)
                + (y - height / 2.0 + top_height + between_buffer + element_height / 2.0),
            TextParams {
                font: state.font_mq,
                font_size: element_height as u16,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                color: WHITE,
            },
        );

        // Connecting
        let l_r_distance = 0.2 * zoom_percentage;
        let centre_loc = Vec2::new(
            left_side,
            get_connector_height(between_buffer, element_height, idx)
                + (y - height / 2.0 + top_height + between_buffer),
        );

        if input::is_mouse_button_released(MouseButton::Left)
            && ((centre_loc.x - l_r_distance) - input::mouse_position().0).abs()
                < 16.0 * zoom_percentage
            && ((centre_loc.y) - input::mouse_position().1).abs() < 16.0 * zoom_percentage
        {
            let node = nodes_clone
                .iter()
                .find(|n| n.output_connectors.iter().find(|i| i.selected).is_some());
            if node.is_some() {
                let other = node
                    .unwrap()
                    .output_connectors
                    .iter()
                    .find(|i| i.selected)
                    .unwrap();

                let a = node_copy
                    .input_connectors
                    .iter()
                    .find(|p| p.connected == Some(node.unwrap().id));

                let a_star = node_copy.input_connectors.iter().find(|p| {
                    p.connected_idx
                        == Some(
                            node_copy
                                .input_connectors
                                .iter()
                                .position(|p| p == c)
                                .unwrap() as u32,
                        )
                });

                let b = match a {
                    Some(n) => {
                        if n.connected_idx
                            != Some(
                                node.unwrap()
                                    .output_connectors
                                    .iter()
                                    .position(|r| r == other)
                                    .unwrap() as u32,
                            )
                        {
                            true
                        } else {
                            false
                        }
                    }
                    None => true,
                };

                if (a.is_none() || a_star.is_none())
                    && b
                    && node
                        .unwrap()
                        .input_connectors
                        .iter()
                        .find(|p| p == &c)
                        .is_none()
                {
                    if node
                        .unwrap()
                        .output_connectors
                        .iter()
                        .find(|r| r.connected == Some(node_copy.id))
                        .is_none()
                        && node
                            .unwrap()
                            .output_connectors
                            .iter()
                            .find(|r| {
                                r.connected_idx
                                    == Some(
                                        node_copy
                                            .input_connectors
                                            .iter()
                                            .position(|p| p == c)
                                            .unwrap()
                                            as u32,
                                    )
                            })
                            .is_none()
                    {
                        if other.connector_type == c.connector_type && c != other {
                            c.connected = Some(node.unwrap().id);
                            c.connected_idx = Some(
                                node.unwrap()
                                    .output_connectors
                                    .iter()
                                    .position(|r| r == other)
                                    .unwrap() as u32,
                            );
                        }
                    }
                }
            }
        }

        if input::is_mouse_button_pressed(MouseButton::Right)
            && ((centre_loc.x - l_r_distance) - input::mouse_position().0).abs()
                < 16.0 * zoom_percentage
            && ((centre_loc.y) - input::mouse_position().1).abs() < 16.0 * zoom_percentage
        {
            c.connected = None;
            c.connected_idx = None;
        }

        if c.connected_idx.is_some() {
            let other_node = nodes_clone
                .iter()
                .find(|p| p.id == c.connected.unwrap())
                .unwrap();
            let other_side = (other_node.position[0] + -state.pos.x) + (width / 2.0);
            let mut other_height: f32 = (other_node.output_connectors.len() as f32
                * element_height)
                + (other_node.input_connectors.len() as f32 * element_height)
                + ((other_node.position.len()
                    + other_node.output_connectors.len()
                    + other_node.input_connectors.len()) as f32
                    * between_buffer);
            other_height *= zoom_percentage;
            let other_top_height: f32 = 150.0 * zoom_percentage;
            other_height += other_top_height;
            other_height *= 2.0;

            draw_line(
                left_side,
                get_connector_height(between_buffer, element_height, idx)
                    + (y - height / 2.0 + top_height + between_buffer),
                other_side,
                get_connector_height(between_buffer, element_height, c.connected_idx.unwrap())
                    + ((other_node.position[1] + -state.pos.y) - other_height / 2.0
                        + top_height
                        + between_buffer),
                2.0,
                WHITE,
            )
        }

        draw_circle(
            left_side,
            get_connector_height(between_buffer, element_height, idx)
                + (y - height / 2.0 + top_height + between_buffer),
            NODE_CIRCLE_RADIUS * zoom_percentage,
            circle_color,
        );

        idx += 1;
    }

    for c in &mut node.output_connectors {
        let mut c_name = c.name.clone();

        let circle_color = match c.connector_type {
            PropertyType::Bool => GREEN,
            PropertyType::Number => BLUE,
            PropertyType::String => RED,
            PropertyType::List => ORANGE,
            PropertyType::File => YELLOW,
            PropertyType::Flow => GRAY,
            PropertyType::Custom(_) => PURPLE,
        };

        let mut _top_text_fits = false;
        let mut top_text_fits_first = true;
        while !_top_text_fits {
            let d = measure_text(&c_name, Some(state.font_mq), element_height as u16, 1.0);
            if d.width < width {
                _top_text_fits = true;
                break;
            }

            top_text_fits_first = false;
            c_name.pop();
        }

        if !top_text_fits_first {
            c_name.pop();
            c_name.pop();
            c_name.pop();
            c_name.pop();
            c_name.push_str("...");
        }

        draw_text_ex(
            &c_name,
            right_side
                - (measure_text(&c_name, Some(state.font_mq), element_height as u16, 1.0).width
                    * 1.1),
            get_connector_height(between_buffer, element_height, idx)
                + (y - height / 2.0 + top_height + between_buffer + element_height / 2.0),
            TextParams {
                font: state.font_mq,
                font_size: element_height as u16,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                color: WHITE,
            },
        );

        // Connecting
        let l_r_distance = 0.2 * zoom_percentage;
        let centre_loc = Vec2::new(
            right_side,
            get_connector_height(between_buffer, element_height, idx)
                + (y - height / 2.0 + top_height + between_buffer),
        );

        if input::is_mouse_button_pressed(MouseButton::Left)
            && ((centre_loc.x - l_r_distance) - input::mouse_position().0).abs()
                < 16.0 * zoom_percentage
            && ((centre_loc.y) - input::mouse_position().1).abs() < 16.0 * zoom_percentage
        {
            c.selected = true;
        }

        if !input::is_mouse_button_down(MouseButton::Left) {
            c.selected = false;
        }

        if c.selected {
            let x = mouse_position().0;
            let y = mouse_position().1;

            draw_line(centre_loc.x, centre_loc.y, x, y, 2.0, WHITE);
        }

        if c.connected_idx.is_some() {
            let other_node = nodes_clone
                .iter()
                .find(|p| p.id == c.connected.unwrap())
                .unwrap();
            let other_side = (other_node.position[0] + -state.pos.x) - (width / 2.0);
            let mut other_height: f32 = (other_node.input_connectors.len() as f32 * element_height)
                + (other_node.output_connectors.len() as f32 * element_height)
                + ((other_node.position.len()
                    + other_node.input_connectors.len()
                    + other_node.output_connectors.len()) as f32
                    * between_buffer);
            other_height *= zoom_percentage;
            let other_top_height: f32 = 150.0 * zoom_percentage;
            other_height += other_top_height;
            other_height *= 2.0;

            draw_line(
                right_side,
                get_connector_height(between_buffer, element_height, idx)
                    + (y - height / 2.0 + top_height + between_buffer),
                other_side,
                get_connector_height(between_buffer, element_height, c.connected_idx.unwrap())
                    + ((other_node.position[1] + -state.pos.y) - other_height / 2.0
                        + top_height
                        + between_buffer),
                2.0,
                WHITE,
            )
        }

        draw_circle(
            right_side,
            get_connector_height(between_buffer, element_height, idx)
                + (y - height / 2.0 + top_height + between_buffer),
            NODE_CIRCLE_RADIUS * zoom_percentage,
            circle_color,
        );

        idx += 1;
    }
}
