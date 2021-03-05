use crate::libs::globals::{FRACTAL_X0, FRACTAL_X1, FRACTAL_Y0, FRACTAL_Y1};

// fractal_** will be constant values, the only thing that changes is zoom. Each call to get will return a value
pub struct Coordinates{
    // pointing to the middle of the screen
    fractal_x: f64,
    // fractal_x1: f64,
    // pointing to the middle of the screen
    fractal_y: f64,
    // fractal_y1: f64,
    width: f64,
    height: f64,

    // These values won't change after the construction of this type.
    default_width: f64,
    default_height: f64,

    // aspect_ratio: f64,
    // starts at 1000. 1010 would mean 1% larger image. 2000 would mean that the image will be twice the size, and so on.
    zoom: u64,
}

impl Default for Coordinates{
    fn default() -> Self{
        let width = FRACTAL_X1 - FRACTAL_X0;
        let height = FRACTAL_Y1 - FRACTAL_Y0;
        // let aspect_ratio = ASPECT_RATIO;

        Coordinates{


            fractal_x: FRACTAL_X0 + (width)/2.0,
            // fractal_x1: FRACTAL_X1,
            fractal_y: FRACTAL_Y0 + (height)/2.0,
            // fractal_y1: FRACTAL_Y1,

            // width: SCREEN_WIDTH as f64,
            // height: SCREEN_HEIGHT as u32,
            width,
            height,

            default_width: width,
            default_height: height,

            // aspect_ratio,
            zoom: 1000,
        }
    }
}

impl Coordinates{
    // pub fn new(fractal_x: f64, fractal_y: f64, zoom: u64) -> Self{
    //     Coordinates{
    //         fractal_x,
    //         fractal_y,
    //         aspect_ratio: ASPECT_RATIO,
    //         zoom,
    //     }
    // }

    pub fn set_pos(&mut self, fractal_x: f64, fractal_y: f64) {
        self.fractal_x = fractal_x;
        self.fractal_y = fractal_y;
        self.update_coords();
    }

    // change the zoom related to the given proportion. zoom == 1.0 => no change.
    pub fn zoom_in_out(&mut self, proportion: f64){
        assert!(proportion >= 0.0, "Error Coordinates::zoom_in_out! proportion ({}) must be positive.", proportion);
        let mut new_zoom = (self.zoom as f64 * proportion) as u64;
        if new_zoom <= 1000 {
            new_zoom = 1000;
        }

        self.zoom = new_zoom;
        self.update_coords();
    }

    /// Moves the cursor to the right proportional to the current width. Proportion must be between 0.0 and 1.0.
    pub fn move_right(&mut self, proportion: f64){
        Self::check_proportion(proportion, String::from("coordinates::Coordinates::move_right"));
        let distance: f64 = self.width * proportion;
        self.fractal_x = self.fractal_x + distance;
        self.update_coords();
    }

    /// Moves the cursor to the left proportional to the current width. Proportion must be between 0.0 and 1.0.
    pub fn move_left(&mut self, proportion: f64){
        Self::check_proportion(proportion, String::from("coordinates::Coordinates::move_left"));
        let distance: f64 = self.width * proportion;
        self.fractal_x = self.fractal_x - distance;
        self.update_coords();
    }

    /// Moves the cursor down proportional to the current height. Proportion must be between 0.0 and 1.0.
    pub fn move_down(&mut self, proportion: f64){
        Self::check_proportion(proportion, String::from("coordinates::Coordinates::move_down"));
        let distance: f64 = self.height * proportion;
        self.fractal_y = self.fractal_y + distance;
        self.update_coords();
    }

    /// Moves the cursor up proportional to the current height. Proportion must be between 0.0 and 1.0.
    pub fn move_up(&mut self, proportion: f64){
        Self::check_proportion(proportion, String::from("coordinates::Coordinates::move_up"));
        let distance: f64 = self.height * proportion;
        self.fractal_y = self.fractal_y - distance;
        self.update_coords();
    }

    pub fn get_coords(&mut self) -> [f64; 4]{
        // let width = FRACTAL_X1 - FRACTAL_X0;
        // let new_width = (width as f64 * 1000.0) as f64/(self.zoom as f64);
        // let new_height = new_width / self.aspect_ratio;
        self.update_coords();

        // The array is built twice for readability. Maybe the compiler will optimize it away.
        let [x0, x1, y0, y1] = 
        [
            self.fractal_x - self.width as f64/ 2.0,
            self.fractal_x + self.width as f64/ 2.0,
            self.fractal_y - self.height as f64/ 2.0,
            self.fractal_y + self.height as f64/ 2.0,
        ];

        [x0, x1, y0, y1]
    }

    /// Updates width and height after each change.
    fn update_coords(&mut self){
        let proportion = 1000.0 / self.zoom as f64;
        self.width = self.default_width * proportion;
        self.height = self.default_height * proportion;
    }

    /// Used for asserting that move and zoom functions have a proportion from 0.0 to 1.0.
    fn check_proportion(proportion: f64, method_name: String){
        assert!((proportion >= 0.0)&&(proportion <= 1.0), "Error {}   proportion ({}) must be between 0 and 1.0", method_name, proportion);
    }
}