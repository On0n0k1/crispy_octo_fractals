// use crate::image_tools::imgcolor::ColorOrder;

// message consts
pub const MESSAGE_SIZE: usize = 2048;
pub const ITERATIONS: u32 = 1024;
// pub const ITERATIONS: u32 = 2048;
// pub const COLOR_ORDER: ColorOrder = ColorOrder::RGB;

// screen consts
// pub const SCREEN_WIDTH: usize = 1024;
pub const SCREEN_WIDTH: usize = 800;
// pub const SCREEN_WIDTH: usize = 3840;
// pub const SCREEN_HEIGHT: usize = 768;
pub const SCREEN_HEIGHT: usize = 600;

pub const ASPECT_RATIO: f64 = SCREEN_WIDTH as f64 / SCREEN_HEIGHT as f64;


// pub const SCREEN_HEIGHT: usize = 2160;
pub const FRACTAL_X0: f64 = -2.5;
// pub const FRACTAL_X0: f64 = -1.249966135;
pub const FRACTAL_X1: f64 = 1.0;
// pub const FRACTAL_X1: f64 = -1.249961638;
pub const FRACTAL_Y0: f64 = -1.5;
// pub const FRACTAL_Y0: f64 = -0.010101873;
// pub const FRACTAL_Y1: f64 = 1.5;
// pub const FRACTAL_Y1: f64 = -0.010098504;
pub const FRACTAL_Y1: f64 = FRACTAL_Y0 + (FRACTAL_X1 - FRACTAL_X0)/ASPECT_RATIO;

// threading consts

// 3840 × 2160