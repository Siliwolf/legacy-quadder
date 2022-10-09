use egui_macroquad::macroquad;
use macroquad::prelude::*;
use quadder::{
    file::{appdata, get_data_store},
    graphics,
};

fn window_config() -> Conf {
    Conf {
        window_title: "Quadder".to_owned(),
        window_width: 1280,
        window_height: 920,
        high_dpi: true,
        fullscreen: false,
        sample_count: 1,
        window_resizable: true,
        // icon: Some(egui_macroquad::macroquad::miniquad::conf::Icon {
        //     small: todo!(),
        //     medium: todo!(),
        //     big: todo!(),
        // }),
        ..Default::default()
    }
}

#[macroquad::main(window_config())]
async fn main() {
    #[cfg(debug_assertions)]
    std::env::set_current_dir(".\\test").unwrap();

    let mut state: graphics::State = graphics::State::new();
    appdata();
    get_data_store();

    loop {
        graphics::draw(&mut state).await;
    }
}
