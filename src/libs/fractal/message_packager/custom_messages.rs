use kik_sync_service::message::{Message, MessageData, MessageInput};
// use bracket_color::prelude;

use crate::libs::globals::{MESSAGE_SIZE, ITERATIONS};
// use crate::image_tools::imgcolor::{Color, ColorOrder};
use crate::libs::fractal::fractal_state::color;
use crate::libs::fractal::fractal_state::run_fractal;


// MessageInput
// This is useful because there is no need to be 1024 elements.
pub struct Position{
    // a message might not be completely filled with inputs.
    pub size: usize,
    pub screen_x: [u32; MESSAGE_SIZE],
    pub screen_y: [u32; MESSAGE_SIZE],

    pub iterations: u32,
    pub fractal_x: [f64; MESSAGE_SIZE],
    pub fractal_y: [f64; MESSAGE_SIZE],
}

impl Default for Position{
    fn default() -> Self{
        Position{
            size: MESSAGE_SIZE,
            screen_x: [0; MESSAGE_SIZE],
            screen_y: [0; MESSAGE_SIZE],

            iterations: ITERATIONS,
            fractal_x: [0.0; MESSAGE_SIZE],
            fractal_y: [0.0; MESSAGE_SIZE],
        }
    }
}

impl Clone for Position{
    fn clone(&self) -> Self{
        Position{
            size: self.size.clone(),
            screen_x: self.screen_x.clone(),
            screen_y: self.screen_y.clone(),
            iterations: self.iterations.clone(),
            fractal_x: self.fractal_x.clone(),
            fractal_y: self.fractal_y.clone(),
        }
    }
}

impl Position{}

impl MessageInput<FractalIntensity> for Position{
    fn new() -> Self {
        Position::default()
    }
}

// MessageData
pub struct FractalIntensity{
    // a message might not be completely filled with inputs.
    pub size: usize,
    pub screen_x: [u32; MESSAGE_SIZE],
    pub screen_y: [u32; MESSAGE_SIZE],

    // The number of iterations for each given point
    pub intensity_value: [u32; MESSAGE_SIZE],
    // message calculates the color of the pixel using the proportion between max and the number of iterations: [R, G, B].
    pub intensity_color: [[u8; 4]; MESSAGE_SIZE],
}

impl Default for FractalIntensity{
    fn default() -> Self{
        FractalIntensity{
            size: MESSAGE_SIZE,
            screen_x: [0; MESSAGE_SIZE],
            screen_y: [0; MESSAGE_SIZE],

            intensity_value: [0; MESSAGE_SIZE],
            intensity_color: [[0; 4]; MESSAGE_SIZE],
        }
    }
}

impl Clone for FractalIntensity{
    fn clone(&self) -> Self{
        FractalIntensity{
            size: self.size.clone(),
            screen_x: self.screen_x.clone(),
            screen_y: self.screen_y.clone(),

            intensity_value: self.intensity_value.clone(),
            intensity_color: self.intensity_color.clone(),
        }
    }
}

impl FractalIntensity{
    /// Changes the value of size. Panics if greater than MESSAGE_SIZE.
    /// There's only need to call this if this is the last message, and there's not enough elements remaining to fill it.
    fn set_size(&mut self, new_size: usize){
        assert!(new_size <= MESSAGE_SIZE, "Error custom_messages::Position::set_size: new_size({}) is greater than the max MESSAGE_SIZE({}) ", new_size, MESSAGE_SIZE);
        self.size = new_size;
    }

    /// Gets a start x0 and an end x1, fills the array with elements between these distances (1 by 1).
    /// Panics if the range is not the same as given size. 
    fn set_screen_x(&mut self, x0: u32, x1: u32){
        assert_eq!(x1 - x0, self.size as u32, "Error custom_messages::Position::set_screen_x: x0({}) - x1({}) == {} which is different from size = {}", x0, x1, x1 - x0, self.size);
        let mut counter = 0;
        for x in x0..x1{
            self.screen_x[counter] = x;
            counter += 1;
        }
    }

    /// Gets a start y0 and an end y1, fills the array with elements between these distances (1 by 1).
    /// Panics if the range is not the same as given size. 
    fn set_screen_y(&mut self, y0: u32, y1: u32){
        assert_eq!(y1 - y0, self.size as u32, "Error custom_messages::Position::set_screen_x: x0({}) - x1({}) == {} which is different from size = {}", y0, y1, y1 - y0, self.size);
        let mut counter = 0;
        for y in y0..y1{
            self.screen_y[counter] = y;
            counter += 1;
        }
    }

    // intensity_value and intensity_color will be set manually by the work function
    // pub fn update_image(&mut self, fractal_intensity: FractalIntensity){
}

impl MessageData for FractalIntensity{
    fn new() -> Self {
        FractalIntensity::default()
    }
}


// Message
// FractalMessage is terrible name, sorry but I'm in a hurry to think of a better.
pub struct FractalMessage{
    message_data: FractalIntensity,
    message_input: Position,

    // Stores what are the main colors and return a color that intensity is determined by how close a value is to the maximum number of iterations.
    // If priority is RGB, the most sensible color is blue, small variations change blue by a lot, and the most intense in high iterations is red. 
    // If priority is GRB, green is the most prevalent in high intensity areas, and so on.
    // color: Color,
}

impl FractalMessage{
    // / Set color based in const global COLOR_ORDER and current number of iterations.
    // / This should be called every time iteration number is changed.
    // fn set_color(&mut self){
    //     let iterations: u32 = self.message_input.iterations;
    //     let color_order = COLOR_ORDER;
    //     self.color = Color::new(color_order, iterations);
    // }

    // / Return a default value for color for creating new instances of message. 
    // / This doesn't implement the default trait because this color default is specific for this crate.
    // fn default_color() -> Color{
    //     Color::new(COLOR_ORDER, ITERATIONS)
    // }
}

impl Default for FractalMessage{
    fn default() -> Self{
        let new_message_input = Position::default();

        FractalMessage{
            message_data: FractalIntensity::default(),
            message_input: Position::default(),
        }
    }
}

impl Clone for FractalMessage{
    fn clone(&self) -> Self{
        FractalMessage{
            message_data: self.message_data.clone(),
            message_input: self.message_input.clone(),
        }
    }
}

impl Message<FractalIntensity, Position> for FractalMessage{
    fn new() -> Self{
        FractalMessage::default()
    }

    fn set_input(&mut self, message_input: Position){
        self.message_input = message_input;
    }

    fn work(&mut self){
        self.message_data.size = self.message_input.size;

        for counter in 0..self.message_input.size{
            self.message_data.screen_x[counter] = self.message_input.screen_x[counter];
            self.message_data.screen_y[counter] = self.message_input.screen_y[counter];
            self.message_data.intensity_value[counter] = run_fractal::run(
                self.message_input.fractal_x[counter],
                self.message_input.fractal_y[counter],
                self.message_input.iterations,
            );
            self.message_data.intensity_color[counter] = color::convertvalue(
                self.message_data.intensity_value[counter]
            );
        }
    }

    fn clone_message_data(&self) -> FractalIntensity{
        self.message_data.clone()
    }

}


