// extern crate rand;
extern crate piston_window;
extern crate image as im;

use piston_window::*;
use crate::libs::globals::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn create_window(img: im::RgbaImage) {
    let mut window: PistonWindow = WindowSettings::new("Piston", [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32]).build().unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    let texture = Texture::from_image(
        &mut texture_context,
        &img,
        &TextureSettings::new(),
    ).unwrap();


    while let Some(e) = window.next() {

        window.draw_2d(&e, |c, g, _|{
            clear([0.0, 0.0, 0.0, 1.0], g);

            image(&texture, c.transform, g);
        });
    }

}
