use crate::libs::globals::{MESSAGE_SIZE, SCREEN_WIDTH, SCREEN_HEIGHT, FRACTAL_X0, FRACTAL_X1, FRACTAL_Y0, FRACTAL_Y1};
use crate::libs::fractal::message_packager::custom_messages;
use crate::libs::fractal::message_packager::coordinates;


pub struct MessagePackager{
    pub fractal_x0: f64,
    pub fractal_x1: f64,
    pub fractal_y0: f64,
    pub fractal_y1: f64,

    coordinates: coordinates::Coordinates,
    // I'm not sure how to implement these values changing yet, so they remain private
    screen_width: usize,
    screen_height: usize,
    message_size: usize,
}
    
impl Default for MessagePackager{
    fn default() -> Self{
        MessagePackager{
            fractal_x0: FRACTAL_X0,
            fractal_x1: FRACTAL_X1,
            fractal_y0: FRACTAL_Y0,
            fractal_y1: FRACTAL_Y1,

            coordinates: coordinates::Coordinates::default(),
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            message_size: MESSAGE_SIZE,
        }
    }
}

// pub struct Position{
//     // a message might not be completely filled with inputs.
//     pub size: usize,
//     pub screen_x: [u32; MESSAGE_SIZE],
//     pub screen_y: [u32; MESSAGE_SIZE],

//     pub iterations: u32,
//     pub fractal_x: [f64; MESSAGE_SIZE],
//     pub fractal_y: [f64; MESSAGE_SIZE],
// }

impl MessagePackager{

    /// Create a new fractal generator with custom coordinates.
    pub fn new(fractal_x: f64, fractal_y: f64) -> Self{

        let mut coordinates = coordinates::Coordinates::default();
        coordinates.set_pos(fractal_x, fractal_y);

        // let [fractal_x0, fractal_x1, fractal_y0, fractal_y1] = coordinates.get_coords();
        // assert!(fractal_x0>=FRACTAL_X0, "fractal_x0 ({}) cannot be lower than the minimum value ({})", fractal_x0, FRACTAL_X0);
        // assert!(fractal_x1<=FRACTAL_X1, "fractal_x1 ({}) cannot be higher than the maximum value ({})", fractal_x1, FRACTAL_X1);
        // assert!(fractal_y0>=FRACTAL_Y0, "fractal_y0 ({}) cannot be lower than the minimum value ({})", fractal_y0, FRACTAL_Y0);
        // assert!(fractal_y1<=FRACTAL_Y1, "fractal_y1 ({}) cannot be higher than the maximum value ({})", fractal_y1, FRACTAL_Y1);

        let mut message_packager = MessagePackager::default();
        // message_packager.fractal_x0 = fractal_x0;
        // message_packager.fractal_x1 = fractal_x1;
        // message_packager.fractal_y0 = fractal_y0;
        // message_packager.fractal_y1 = fractal_y1;
        message_packager.coordinates = coordinates;
        message_packager.update_pos();
        message_packager
    }

    pub fn set_pos(&mut self, fractal_x: f64, fractal_y: f64){
        self.coordinates.set_pos(fractal_x, fractal_y);
        self.update_pos();
    }

    // change the zoom related to the given proportion. zoom == 1.0 => no change.
    pub fn zoom_in_out(&mut self, proportion: f64){
        self.coordinates.zoom_in_out(proportion);
        self.update_pos();
    }
    pub fn move_left(&mut self, proportion:f64){
        self.coordinates.move_left(proportion);
        self.update_pos();
    }
    pub fn move_right(&mut self, proportion:f64){
        self.coordinates.move_right(proportion);
        self.update_pos();
    }
    pub fn move_down(&mut self, proportion:f64){
        self.coordinates.move_down(proportion);
        self.update_pos();
    }
    pub fn move_up(&mut self, proportion:f64){
        self.coordinates.move_up(proportion);
        self.update_pos();
    }

    pub fn inputs_into_vec(&mut self, iterations: u32) -> Vec<custom_messages::Position>{
        // How many elements are in total
        let total_elements = self.screen_width * self.screen_height;
        let remainder = total_elements % self.message_size;

        let total_messages: usize;
        // last message may not be full
        if remainder > 0{
            total_messages = (total_elements - remainder) / self.message_size + 1;
        } else {
            total_messages = (total_elements - remainder) / self.message_size;
        }

        // fractal distances between each pixel in the screen
        let x_scale: f64 = (self.fractal_x1 - self.fractal_x0) / (self.screen_width as f64);
        let y_scale: f64 = (self.fractal_y1 - self.fractal_y0) / (self.screen_height as f64); 

        let mut new_vec: Vec<custom_messages::Position> = Vec::with_capacity(total_messages);

        // function will recycle these arrays, and only clone them when creating Position
        let mut new_screen_x: [u32; MESSAGE_SIZE] = [0; MESSAGE_SIZE];
        let mut new_screen_y: [u32; MESSAGE_SIZE] = [0; MESSAGE_SIZE];
        let mut new_fractal_x: [f64; MESSAGE_SIZE] = [0.0; MESSAGE_SIZE];
        let mut new_fractal_y: [f64; MESSAGE_SIZE] = [0.0; MESSAGE_SIZE];

        for counter_message in 0..total_messages{
            let new_size: usize;
            if counter_message < total_messages - 1 {
                new_size = self.message_size;
            } else {
                // if there is remainder from the calculation up there, last message will be just these remaining values
                if remainder != 0{
                    new_size = remainder;
                } else {
                    new_size = self.message_size;
                }
            }

            // Code is built in a way that new_size sho uld never be larger than MESSAGE_SIZE
            for counter_element in 0..MESSAGE_SIZE{
                if counter_element >= new_size{
                    // cleaning all unused values from previous iterations. This happens only in last message.
                    new_screen_x[counter_element] = 0;
                    new_screen_y[counter_element] = 0;
                    new_fractal_x[counter_element] = 0.0;
                    new_fractal_y[counter_element] = 0.0;
                    continue
                }

                // There's a difference between how each element is distributed among lines and messages. But both have the same number of elements.
                // This counts all elements in a single line, before converting them into the "screen" position, and finally, "fractal" position.
                let counter = counter_message * self.message_size + counter_element;

                new_screen_x[counter_element] = counter as u32 % self.screen_width as u32;
                new_screen_y[counter_element] = (counter as u32 - new_screen_x[counter_element] as u32) / self.screen_width as u32;
                new_fractal_x[counter_element] = (new_screen_x[counter_element] as f64) * x_scale + self.fractal_x0;
                new_fractal_y[counter_element] = (new_screen_y[counter_element] as f64) * y_scale + self.fractal_y0;
            }

            // each element of the vector will have a slice of the screen for the worker threads to work on.
            new_vec.push(
                custom_messages::Position{
                    size: new_size,
                    screen_x: new_screen_x.clone(),
                    screen_y: new_screen_y.clone(),

                    iterations: iterations.clone(),
                    fractal_x: new_fractal_x.clone(),
                    fractal_y: new_fractal_y.clone(),
                }
            );                                               
        };

        new_vec
    }

    fn update_pos(&mut self){
        let [fractal_x0, fractal_x1, fractal_y0, fractal_y1] = self.coordinates.get_coords();
        self.fractal_x0 = fractal_x0;
        self.fractal_x1 = fractal_x1;
        self.fractal_y0 = fractal_y0;
        self.fractal_y1 = fractal_y1;
    }
}

