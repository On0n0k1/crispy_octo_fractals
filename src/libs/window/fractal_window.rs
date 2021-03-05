// extern crate rand;
extern crate piston_window;
extern crate image as im;

use piston_window::*;
// use crate::libs::globals::{SCREEN_WIDTH, SCREEN_HEIGHT};



use kik_sync_service::channel::DeliveryService;

use crate::libs::globals::{SCREEN_WIDTH, SCREEN_HEIGHT, ITERATIONS};
use crate::libs::fractal::message_packager::message_packager::MessagePackager;
use crate::libs::fractal::message_packager::custom_messages::{FractalMessage, FractalIntensity, Position};
use crate::libs::fractal::image_tools::ImgTools;


pub struct FractalWindow{
    delivery_service: DeliveryService<FractalIntensity, Position, FractalMessage>,
    message_packager: MessagePackager,
    image_tools: ImgTools,
    move_rate: f64,
}

impl Default for FractalWindow{
    fn default() -> Self{
        let delivery_service: DeliveryService<FractalIntensity, Position, FractalMessage> = DeliveryService::default();
        let message_packager = MessagePackager::default();
        let mut path_image = std::env::current_dir().unwrap();
        path_image.push("test.png");
        let image_tools = ImgTools::new(SCREEN_HEIGHT as u32, SCREEN_WIDTH as u32, path_image);

        FractalWindow{
            delivery_service,
            message_packager,
            image_tools,
            move_rate: 0.1,
        }
    }
}

impl FractalWindow{
    // pub fn create_window(&mut self, img: im::RgbaImage) {
    pub fn create_window(&mut self) {
        let mut window: PistonWindow = WindowSettings::new("Piston", [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32]).build().unwrap();

        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into()
        };

        let mut texture = Texture::from_image(
            &mut texture_context,
            &self.image_tools.img,
            &TextureSettings::new(),
        ).unwrap();

        self.update_image();


        while let Some(e) = window.next() {

            if let Some(button) = e.release_args(){
                match button {
                    Button::Keyboard(key) =>{
                        match key{
                            Key::W => self.release_w(),
                            Key::A => self.release_a(),
                            Key::S => self.release_s(),
                            Key::D => self.release_d(),
                            Key::Q => self.release_q(),
                            Key::E => self.release_e(),
                            Key::Escape => {
                                println!("Pressed escape. Closing...");
                                window.set_should_close(true);
                            },
                            _ => {},
                        }
                    },
                    _ =>{},
                }
            }

            window.draw_2d(&e, |c, g, _|{


                // What I want to do:
                // W A S D movement keys
                // Q E zoom in out
                // + - increment decrement rate of zoom
                // esc quit
                // enter save image

                

                println!("Clearing screen");
                clear([0.0, 0.0, 0.0, 1.0], g);
                texture = Texture::from_image(
                    &mut texture_context,
                    &self.image_tools.img,
                    &TextureSettings::new(),
                ).unwrap();
                
                println!("Showing image");
                image(&texture, c.transform, g);
            });
        }

    }

    fn update_image(&mut self){
        println!("Updating Image");
        let mut feeder_vec = self.message_packager.inputs_into_vec(1024);
        self.delivery_service.feed_feeder(&mut feeder_vec);
        for package in &mut self.delivery_service{
            self.image_tools.update_image(package);
        }
    }

    fn release_w(&mut self){
        // Move 20% of the screen up
        println!("Pressed up");
        self.message_packager.move_up(self.move_rate);
        self.update_image();
    }

    fn release_s(&mut self){
        println!("Pressed down");
        self.message_packager.move_down(self.move_rate);
        self.update_image();
    }

    fn release_a(&mut self){
        println!("Pressed left");
        self.message_packager.move_left(self.move_rate);
        self.update_image();
    }

    fn release_d(&mut self){
        println!("Pressed right");
        self.message_packager.move_right(self.move_rate);
        self.update_image();
    }

    fn release_q(&mut self){
        println!("Pressed zoom_out Q");
        self.message_packager.zoom_in_out(0.5);
        self.update_image();
    }

    fn release_e(&mut self){
        println!("Pressed zoom_in E");
        self.message_packager.zoom_in_out(1.5);
        self.update_image();
    }

}