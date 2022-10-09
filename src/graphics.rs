use egui_macroquad::egui::{self, Align2, Color32, Rounding, Style};
use egui_macroquad::macroquad::{self, hash, input, math::Vec2};
use macroquad::prelude::*;

use crate::*;
use crate::file::SaveFile;
use crate::log::log_warning;
use crate::textbox::text_to_type;
use crate::{Node, NodeConnector, NodeType, PropertyType, textbox, List, editor, MAX_ZOOM};

#[derive(PartialEq, Clone)]
pub enum Screen {
    MENU,
    EDITOR,
    SETTINGS,
    NEWPROJ,
}

#[derive(Clone)]
pub struct State {
    pub screen: Screen,
    pub open_file: Option<SaveFile>,
    pub theme: Style,
    pub zoom: f32,
    pub pos: Vec2,
    pub minus_button_debounce: bool,
    pub plus_button_debounce: bool,
    pub last_mouse_pos: Vec2,
    pub interactive: bool,
    pub font_mq: Font,
    // Used by draw functions \/
    pub last_mouse_click: Vec2,
    pub egui_over: bool,
    pub newproj_dialog_open: bool,
    // Used by draw functions \/
    pub selected_node: Option<Node>,
    // Used by draw functions \/
    pub prev_movement_selected_node: Option<Node>,
    pub cur_comment_open: bool,
    pub cur_comment: String,
    pub cur_const_type: Option<PropertyType>,
    pub cur_const_data: Option<String>,
    pub cur_var_type: Option<PropertyType>,
    pub cur_var_data: Option<String>,

    pub save_dialog_open: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            screen: Screen::MENU,
            open_file: None,
            theme: Style::default(),
            zoom: 15.0,
            pos: Vec2::new(0.0, 0.0),
            minus_button_debounce: false,
            plus_button_debounce: true,
            last_mouse_pos: Vec2::new(0.0, 0.0),
            interactive: false,
            font_mq: load_ttf_font_from_bytes(include_bytes!("./resources/font0.ttf")).unwrap(),
            last_mouse_click: Vec2::new(-1., -1.),
            selected_node: None,
            prev_movement_selected_node: None,
            egui_over: false,
            newproj_dialog_open: false,
            cur_comment_open: false,
            cur_comment: String::new(),
            cur_const_data: None,
            cur_const_type: None,
            cur_var_data: None,
            cur_var_type: None,

            save_dialog_open: false,
        }
    }
}

pub async fn draw(state: &mut State) {
    clear_background(Color {
        r: 0.15,
        g: 0.15,
        b: 0.151,
        a: 1.0,
    });

    // Process keys, mouse etc.
    if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)) && is_key_pressed(KeyCode::S) {
        println!("fs");
        state.save_dialog_open = true;
    }

    // Zoom
    if !state.egui_over {
        if state.zoom < MAX_ZOOM && (is_key_pressed(KeyCode::Equal) || input::mouse_wheel().1 > 0.1)
        {
            state.zoom += ZOOM_DELTA;

            if state.selected_node.is_some() {
                let node = state.selected_node.as_ref().unwrap().clone();
                let x = node.position[0] + -state.pos[0];
                let y = node.position[1] + -state.pos[1];
                state.last_mouse_click = Vec2::new(x, y);
            }
        }

        if state.zoom > MIN_ZOOM
            && (is_key_pressed(KeyCode::Minus) || input::mouse_wheel().1 < -0.1)
        {
            state.zoom += -ZOOM_DELTA;

            if state.selected_node.is_some() {
                let node = state.selected_node.as_ref().unwrap().clone();
                let x = node.position[0] + -state.pos[0];
                let y = node.position[1] + -state.pos[1];
                state.last_mouse_click = Vec2::new(x, y);
            }
        }
    }

    // Pos
    let mouse_delta =
        state.last_mouse_pos - Vec2::new(input::mouse_position().0, input::mouse_position().1);
    state.last_mouse_pos = Vec2::new(input::mouse_position().0, input::mouse_position().1);
    if input::is_mouse_button_down(MouseButton::Middle) {
        state.pos += mouse_delta;
        if state.selected_node.is_some() {
            let node = state.selected_node.as_ref().unwrap().clone();
            let x = node.position[0] + -state.pos[0];
            let y = node.position[1] + -state.pos[1];
            state.last_mouse_click = Vec2::new(x, y);
        }
    }

    if !state.interactive {
        state.zoom = 15.0;
        state.pos = Vec2::new(0.0, 0.0);
    }

    // Last Mouse Click
    if input::is_mouse_button_pressed(MouseButton::Left) {
        state.last_mouse_click = Vec2::new(input::mouse_position().0, input::mouse_position().1);
    }

    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_style(get_style());

        state.egui_over = egui_ctx.is_pointer_over_area();

        egui::Window::new("Menu Bar")
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .title_bar(false)
            .scroll2([false, false])
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.columns(4, |ui| {
                    ui[0].label("Quadder");

                    ui[1].menu_button("General", |ui| {});

                    ui[2].menu_button("Add", |ui| {});

                    ui[3].menu_button("Build", |ui| {});
                });
            });
        
        // Save GUI
        /*
        if state.save_dialog_open {
            if state.open_file.is_some() {
                log::log("Attempting to save file...".to_owned());

                if file::SaveFile::exists(state.clone().open_file.unwrap().name)
                {
                    state.clone().open_file.unwrap().save();
                    state.save_dialog_open = false;
                }
                else {
                    egui::Window::new("Save File")
                    .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                    .title_bar(true)
                    .scroll2([false, false])
                    .resizable(false)
                    .show(egui_ctx, |ui| {
                        let mut str_data = state.open_file.clone().unwrap().name;
                        ui.label("Enter file name:");
                        ui.text_edit_singleline(&mut str_data);
                        let open_file = state.open_file.clone().unwrap();
                        state.open_file = Some(SaveFile {
                            latest_id: open_file.latest_id,
                            name: str_data,
                            nodes: open_file.nodes,
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                state.open_file.clone().unwrap().save();
                                state.save_dialog_open = false;
                            }
                            if ui.button("Close").clicked() {
                                state.save_dialog_open = false;
                            }
                        });
                    });
                }
            }
            else {
                log::log_warning("Could not save as there is no open file".to_owned());
            }
        }

        // Comment GUI
        if state.cur_comment_open {
            egui::Window::new("Edit Comment")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .scroll2([false, true])
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    let mut str_data = state.cur_comment.clone();
                    ui.centered_and_justified(|ui| {
                        ui.text_edit_multiline(&mut str_data);
                        state.cur_comment = str_data;

                        if ui.button("Done").clicked() {
                            state.cur_comment_open = false;
                        }
                    });
                });
        }*/

        // Variable GUI
        if state.cur_var_type.is_some() {
            egui::Window::new("Edit Variable")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .resizable(false)
            .scroll2([false, true])
            .collapsible(false)
            .show(egui_ctx, |ui| {
                let mut cur_type = state.cur_var_type.clone().unwrap();
                let mut cur_name = state.cur_var_data.clone().unwrap();
                let mut custom_bool = match cur_type.clone() {
                    PropertyType::Custom(_) => true,
                    _ => false,
                };

                ui.label(egui::RichText::new("Variable Name").size(24.0));
                ui.label(egui::RichText::new("Note: case insensitive").size(12.0));
                ui.text_edit_singleline(&mut cur_name);

                ui.label(egui::RichText::new("Variable Type").size(24.0));

                ui.horizontal(|ui| {
                    if ui.selectable_label(!custom_bool, "Standard").clicked() {
                        custom_bool = false;
                        cur_type = PropertyType::String;
                    }

                    if ui.selectable_label(custom_bool, "Custom").clicked() {
                        custom_bool = true;
                    }
                });
                
                if custom_bool {
                    let mut type_str = match cur_type.clone() {
                        PropertyType::Custom(s) => s,
                        _ => String::new()
                    };
                    ui.text_edit_singleline(&mut type_str);
                    cur_type = PropertyType::Custom(type_str);
                }
                else {
                    egui::ComboBox::from_label("Choose Type").selected_text(cur_type.to_string()).show_ui(ui, |ui| {
                        ui.selectable_value(&mut cur_type, PropertyType::Bool, "Boolean".to_owned());
                        ui.selectable_value(&mut cur_type, PropertyType::String, "String".to_owned());
                        ui.selectable_value(&mut cur_type, PropertyType::List, "List".to_owned());
                        ui.selectable_value(&mut cur_type, PropertyType::Number, "Number".to_owned());
                        ui.selectable_value(&mut cur_type, PropertyType::File, "File".to_owned());
                    });
                }
                
                state.cur_var_type = Some(cur_type);
                state.cur_var_data = Some(cur_name);

                if ui.button("Done").clicked() {
                    state.cur_var_type = None
                }
            });
        }

        // Constant GUI
        if state.cur_const_type.is_some() {
            egui::Window::new("Edit Constant")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .scroll2([false, true])
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    let mut cur_type = state.cur_const_type.clone().unwrap();
                    let mut custom_bool = matches!(cur_type, PropertyType::Custom(_));
                    let custom_bool_clone = custom_bool.clone();

                    ui.horizontal(|ui| {
                        if ui.selectable_label(!custom_bool, "Standard").clicked() {
                            custom_bool = false;
                        }

                        if ui.selectable_label(custom_bool, "Custom").clicked() {
                            custom_bool = true;
                        }
                    });

                    if custom_bool {
                        if custom_bool_clone == false {
                            cur_type = PropertyType::Custom(String::new());
                        }

                        match &mut cur_type.clone() {
                            PropertyType::Custom(raw_data) => {
                                ui.text_edit_singleline(raw_data);
                                cur_type = PropertyType::Custom(raw_data.to_owned());
                            },
                            _ => {}
                        }

                    } else {
                        if custom_bool_clone == true {
                            cur_type = PropertyType::String;
                        }

                        egui::ComboBox::from_label("Choose Type").selected_text(cur_type.to_string()).show_ui(ui, |ui| {
                            ui.selectable_value(&mut cur_type, PropertyType::Bool, "Boolean".to_owned());
                            ui.selectable_value(&mut cur_type, PropertyType::String, "String".to_owned());
                            ui.selectable_value(&mut cur_type, PropertyType::List, "List".to_owned());
                            ui.selectable_value(&mut cur_type, PropertyType::Number, "Number".to_owned());
                            ui.selectable_value(&mut cur_type, PropertyType::File, "File".to_owned());
                        });

                        match cur_type {
                            PropertyType::Bool => {
                                ui.separator();

                                let cur_const_data = state.cur_const_data.clone();
                                let mut checked = match cur_const_data.unwrap().as_str() {
                                    "true" => {
                                        true
                                    }
                                    "false" => {
                                        false
                                    }
                                    _ => {
                                        false
                                    }
                                };
                                ui.checkbox(&mut checked, "");
                                
                                if checked {
                                    state.cur_const_data = Some("true".to_owned());
                                }
                                else {
                                    state.cur_const_data = Some("false".to_owned());
                                }
                            },
                            PropertyType::Number => {
                                ui.separator();

                                let mut data = textbox::text_to_type(state.cur_const_data.clone().unwrap(), PropertyType::Number);

                                ui.text_edit_singleline(&mut data);

                                state.cur_const_data = Some(data);
                            },
                            PropertyType::String => {
                                ui.separator();

                                let mut data = state.cur_const_data.clone().unwrap();

                                ui.text_edit_multiline(&mut data);

                                state.cur_const_data = Some(data);
                            },
                            PropertyType::List => {
                                // Deserialization
                                let json_input: Result<List, serde_json::Error> = serde_json::from_str(&state.cur_const_data.as_ref().unwrap());
                                let mut list = match json_input {
                                    Ok(i) => {
                                        i
                                    },
                                    Err(_) => {
                                        List {
                                            list_items: Vec::new(),
                                            list_type: PropertyType::Bool,
                                        }
                                    },
                                };

                                ui.separator();

                                let custom_type = match list.clone().list_type {
                                    PropertyType::Custom(t) => {
                                        warn!("Custom Type");
                                        t
                                    }
                                    _ => {
                                        String::new()
                                    }
                                };

                                egui::ComboBox::from_label("List Type").selected_text(list.list_type.to_string()).show_ui(ui, |ui| {
                                    ui.selectable_value(&mut list.list_type, PropertyType::String, "String".to_owned());
                                    ui.selectable_value(&mut list.list_type, PropertyType::Number, "Number".to_owned());
                                    ui.selectable_value(&mut list.list_type, PropertyType::Custom(custom_type.to_string()), "Custom".to_owned());
                                });

                                {
                                    let mut list_clone = list.clone();
                                    let list_immut = list.clone();
                                    match list.clone().list_type {
                                        PropertyType::Number => {
                                            let mut idx = 0;
                                            let mut remove_queue: Vec<usize> = vec![];
                                            while idx < list_immut.list_items.len() {
                                                ui.horizontal(|ui| {
                                                    // Editing
                                                    let mut text = list_immut.list_items[idx].clone();
                                                    text = text_to_type(text, PropertyType::Number);
                                                    ui.text_edit_singleline(&mut text);
                                                    // Don't know why this needs to be there, but doesn't work unless it is
                                                    format!("{:?}", std::mem::replace(&mut list_clone.list_items[idx as usize], text.clone()));
                                                    
                                                    list_clone.list_items[idx as usize] = std::mem::replace(&mut list_clone.list_items[idx as usize], text);
                                                    

                                                    // Adding to remove queue
                                                    if ui.small_button("X").clicked() {
                                                        remove_queue.push(idx);
                                                    }
                                                });
                                                idx += 1;
                                            }

                                            for u in remove_queue {
                                                list_clone.list_items.remove(u);
                                            }
                                        }
                                        PropertyType::String => {
                                            let mut idx = 0;
                                            let mut remove_queue: Vec<usize> = vec![];
                                            while idx < list_immut.list_items.len() {
                                                ui.horizontal(|ui| {
                                                    // Editing
                                                    let mut text = list_immut.list_items[idx].clone();
                                                    ui.text_edit_singleline(&mut text);
                                                    // Don't know why this needs to be there, but doesn't work unless it is
                                                    format!("{:?}", std::mem::replace(&mut list_clone.list_items[idx as usize], text.clone()));
                                                    
                                                    list_clone.list_items[idx as usize] = std::mem::replace(&mut list_clone.list_items[idx as usize], text);
                                                    

                                                    // Adding to remove queue
                                                    if ui.small_button("X").clicked() {
                                                        remove_queue.push(idx);
                                                    }
                                                });
                                                idx += 1;
                                            }

                                            for u in remove_queue {
                                                list_clone.list_items.remove(u);
                                            }
                                        }
                                        PropertyType::Custom(t) => {
                                            // Do once I have the infrastructure
                                            format!("{}", t);
                                            warn!("Custom Type")
                                        }
                                        _ => {}
                                    }

                                    // Adding
                                    if ui.button("Add Item").clicked() {
                                        list_clone.list_items.push(String::new());
                                    }

                                    list = list_clone;
                                }

                                // Serialization
                                let json_output = serde_json::to_string(&list);
                                state.cur_const_data = Some(
                                    match json_output {
                                        Ok(o) => {
                                            o
                                        },
                                        Err(e) => {
                                            log_warning(format!("{:?}", e));
                                            String::new()
                                        },
                                    }
                                )
                            },
                            PropertyType::File => {
                                /*ui.separator();

                                let mut path = state.cur_const_data.clone().unwrap();
                                if !std::path::Path::new(&path.clone()).exists() {
                                    ui.colored_label(Color32::RED, "File is invalid");
                                }

                                ui.horizontal(|ui| {
                                    if ui.button("Open File").clicked() {
                                        let file = native_dialog::FileDialog::new().show_open_single_file();
                                        if file.is_ok() {
                                            if file.as_ref().unwrap().is_some() {
                                                path = file.unwrap().unwrap().as_path().to_str().unwrap().to_string();
                                            }
                                        }
                                    }

                                    if ui.button("Clear").clicked() {
                                        path = String::new();
                                    }
                                });

                                ui.label(path.clone());

                                state.cur_const_data = Some(path);*/
                            },
                            PropertyType::Flow => {
                                state.cur_const_type = Some(PropertyType::Number)
                            },
                            PropertyType::Custom(_) => {},
                        }
                    }

                    state.cur_const_type = Some(cur_type);

                    if ui.button("Done").clicked() {
                        state.cur_const_type = None;
                        state.cur_const_data = None;
                    }
                });
        }

        if state.screen == Screen::MENU {
            state.interactive = false;
            egui::Window::new(
                "                                 Quadder                                   ",
            )
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .resizable(false)
            .scroll2([false, true])
            .collapsible(false)
            .show(egui_ctx, |ui| {
                ui.columns(2, |ui| {
                    // Recent projects
                    ui[0].label("Recent Projects");

                    // Other stuff
                    if ui[1].button("New Project").clicked() {
                        state.screen = Screen::EDITOR;
                        state.interactive = true;
                        state.open_file = Some(SaveFile::new());

                        state.open_file = Some(SaveFile {
                            name: "Debug".to_owned(),
                            nodes: [
                                Node {
                                    name: "Test1".to_owned(),
                                    id: 0,
                                    node_type: NodeType::Function,
                                    position: [700.0, 600.0],
                                    input_connectors: [].to_vec(),
                                    output_connectors: [
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "TestOut".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "TestOutTestOutTestOutTestOutTestOutTestOutTestOutTestOut".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                    ]
                                    .to_vec(),
                                    selected: false,
                                },
                                Node {
                                    name: "Test2".to_owned(),
                                    id: 1,
                                    node_type: NodeType::Function,
                                    position: [400.0, 500.0],
                                    input_connectors: [
                                        NodeConnector {
                                            connector_type: PropertyType::String,
                                            connected: None,
                                            connected_idx: None,
                                            name:
                                                "Test1Test1Test1Test1Test1Test1Test1Test1Test1Test1"
                                                    .to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "Test2".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "Test2".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                    ]
                                    .to_vec(),
                                    selected: false,
                                    output_connectors: [
                                        NodeConnector {
                                            connector_type: PropertyType::String,
                                            connected: None,
                                            connected_idx: None,
                                            name:
                                                "Test1Test1Test1Test1Test1Test1Test1Test1Test1Test1"
                                                    .to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "Test2".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "Test2".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                    ].to_vec(),
                                },
                                Node {
                                    name: "0".to_owned(),
                                    id: 2,
                                    node_type: NodeType::Variable,
                                    position: [1000.0, 600.0],
                                    input_connectors: [].to_vec(),
                                    output_connectors: [
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "TestOut".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                        NodeConnector {
                                            connector_type: PropertyType::Number,
                                            connected: None,
                                            connected_idx: None,
                                            name: "TestOutTestOutTestOutTestOutTestOutTestOutTestOutTestOut".to_owned(),
                                            selected: false,
                                            id: hash!(),
                                        },
                                    ]
                                    .to_vec(),
                                    selected: false,
                                },
                            ]
                            .to_vec(),
                            latest_id: 2,
                        });
                    }

                    if ui[1].button("Open Project From Drive").clicked() {}

                    if ui[1].button("Open Documentation").clicked() {}

                    if ui[1].button("Open Settings").clicked() {
                        state.screen = Screen::SETTINGS;
                    }
                })
            });
        }

        if state.screen == Screen::SETTINGS {
            state.interactive = false;
            egui::Window::new("Settings")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .scroll2([false, true])
                .collapsible(false)
                .title_bar(true)
                .show(egui_ctx, |ui| {
                    if ui.button("Back to menu").clicked() {
                        state.screen = Screen::MENU;
                    }
                });
        }
    });

    // Draw things before egui
    editor::mq_draw_editor(state);

    // Legacy save code

    // if state.screen == Screen::EDITOR {
    //     // Save
    //     if (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
    //         && is_key_down(KeyCode::S)
    //     {
    //         if state.open_file.is_some() {
    //             let data: &mut SaveFile = state.open_file.as_mut().unwrap();

    //             if SaveFile::exists(data.name.clone()) {
    //                 state.open_file = Some(data.clone());
    //             } else {
    //                 println!("Could not save file");
    //             }
    //         }
    //     }
    // }

    egui_macroquad::draw();

    // Draw things after egui

    next_frame().await;
}

fn get_style() -> Style {
    let mut style = Style::default();
    style.visuals.dark_mode = true;
    style.visuals.window_rounding = Rounding {
        nw: 0.0,
        ne: 0.0,
        sw: 0.0,
        se: 0.0,
    };
    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgba_premultiplied(47, 49, 54, 220);
    style.visuals.extreme_bg_color = Color32::from_rgba_premultiplied(79, 79, 85, 220);
    style.visuals.window_shadow.extrusion = 0.0;
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgba_premultiplied(53, 54, 50, 220);

    egui_macroquad::cfg(|ctx| {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "Font0".to_owned(),
            egui::FontData::from_static(include_bytes!("./resources/font0.ttf")).tweak(
                egui::FontTweak {
                    scale: 2.0,
                    y_offset_factor: 0.0,
                    y_offset: -8.0,
                },
            ),
        );

        fonts.families.insert(
            egui::FontFamily::Name("Font0".into()),
            vec!["Font0".to_owned()],
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "Font0".to_owned());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "Font0".to_owned());

        ctx.set_fonts(fonts);
    });

    style
}
