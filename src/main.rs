use egui_macroquad::macroquad;
use macroquad::prelude::*;
use quadder::{
    graphics, github,
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
    let mut state: graphics::State = graphics::State::new();

    println!("{}", github::get_link().await);
    println!("{}", github::get_token().await);

    loop {
        graphics::draw(&mut state).await;
    }
}
