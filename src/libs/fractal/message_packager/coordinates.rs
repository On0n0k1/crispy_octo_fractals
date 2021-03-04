use crate::libs::globals::{ASPECT_RATIO, FRACTAL_X0, FRACTAL_X1, FRACTAL_Y0, FRACTAL_Y1};

// fractal_** will be constant values, the only thing that changes is zoom. Each call to get will return a value
pub struct Coordinates{
    // pointing to the middle of the screen
    fractal_x: f64,
    // fractal_x1: f64,
    // pointing to the middle of the screen
    fractal_y: f64,
    // fractal_y1: f64,

    aspect_ratio: f64,
    // starts at 1000. 1010 would mean 1% larger image. 2000 would mean that the image will be twice the size, and so on.
    zoom: u64,
}

impl Default for Coordinates{
    fn default() -> Self{
        Coordinates{
            fractal_x: FRACTAL_X0 + (FRACTAL_X1 - FRACTAL_X0)/2.0,
            // fractal_x1: FRACTAL_X1,
            fractal_y: FRACTAL_Y0 + (FRACTAL_Y1 - FRACTAL_Y0)/2.0,
            // fractal_y1: FRACTAL_Y1,

            aspect_ratio: ASPECT_RATIO,
            zoom: 1000,
        }
    }
}

impl Coordinates{
    pub fn new(fractal_x: f64, fractal_y: f64, zoom: u64) -> Self{
        Coordinates{
            fractal_x,
            fractal_y,
            aspect_ratio: ASPECT_RATIO,
            zoom,
        }
    }

    pub fn set_pos(&mut self, fractal_x: f64, fractal_y: f64) {
        self.fractal_x = fractal_x;
        self.fractal_y = fractal_y;
    }

    // change the zoom related to the given proportion. zoom == 1.0 => no change.
    pub fn zoom_in_out(&mut self, proportion: f64){
        assert!(proportion >= 0.0, "Error Coordinates::zoom_in_out! proportion ({}) must be positive.", proportion);
        let mut new_zoom = (self.zoom as f64 * proportion) as u64;
        if new_zoom <= 1000 {
            new_zoom = 1000;
        }

        self.zoom = new_zoom;
    }

    pub fn get_coords(&self) -> [f64; 4]{
        let width = FRACTAL_X1 - FRACTAL_X0;
        let new_width = (width as f64 * 1000.0) as f64/(self.zoom as f64);
        let new_height = new_width / self.aspect_ratio;

        // The array is built twice for readability. Maybe the compiler will optimize it away.
        let [x0, x1, y0, y1] = 
        [
            self.fractal_x - new_width / 2.0,
            self.fractal_x + new_width / 2.0,
            self.fractal_y - new_height / 2.0,
            self.fractal_y + new_height / 2.0,
        ];

        [x0, x1, y0, y1]
    }
}