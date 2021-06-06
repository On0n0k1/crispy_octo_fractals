extern crate kik_sync_service;

mod libs;

// use kik_sync_service::channel::DeliveryService;

// use libs::globals::{SCREEN_WIDTH, SCREEN_HEIGHT, ITERATIONS};
// // use libs::fractal::message_packager::message_packager::MessagePackager;
// use libs::fractal::message_packager::custom_messages::{FractalMessage, FractalIntensity, Position};
// use libs::fractal::image_tools::ImgTools;

use libs::window::fractal_window::FractalWindow;


fn main() {

    // let mut message_packager = MessagePackager::default();
    // let mut delivery_service: DeliveryService<FractalIntensity, Position, FractalMessage> = DeliveryService::default();
    // message_packager.zoom_in_out(300.0);
    // message_packager.zoom_in_out(2.0);
    // message_packager.set_pos(0.29, 0.015);
    // let mut path_image = std::env::current_dir().unwrap();
    // path_image.push("test.png");
    // let mut img_tools = ImgTools::new(SCREEN_HEIGHT as u32, SCREEN_WIDTH as u32, path_image);

    // let mut feeder_vec = message_packager.inputs_into_vec(1024);
    // delivery_service.feed_feeder(&mut feeder_vec);
    // for package in &mut delivery_service{
    //     img_tools.update_image(package);
    // }

    // img_tools.save_image();
    let mut window = FractalWindow::default();
    window.create_window();

    // img_tools.create_window();
}
