use std::path::PathBuf;

// use std::f64::consts::PI;
use image:: {RgbaImage, Rgba};
// use crate::global_consts::{SCREENHEIGHT, WIDTH_PER_THREAD};
use crate::libs::fractal::message_packager::custom_messages::FractalIntensity;
use crate::libs::fractal::fractal_state::color::convertvalue;
// use crate::libs::globals::ITERATIONS;

// use crate::libs::fractal::image_tools::imgcolor::{Color, ColorOrder};

use crate::libs::window::fractal_window::create_window;

/// Handles image reading and printing.
pub struct ImgTools{
    height: u32,
    width: u32,
    path: PathBuf,
    img: image::RgbaImage,
}

impl ImgTools{
    /// Create new ImgTools.
    /// height of the image to save. Will panic if it tries to access a pixel out of bounds.
    /// width of the image to save. Will panic if it tries to access a pixel out of bounds.
    /// path of the file to save. Must have something like .png at the end for file format. (I should improve this part)
    /// colororder is a string that tells the priority of colors. Can be RGB RBG BRG BGR GRB or GBR.
    /// iterations is the max value that the intensity can have.
    pub fn new(height: u32, width: u32, path: PathBuf, iterations: u32) -> Self{
        let mut img = RgbaImage::new(width,height);
        for y in 0..height{
            for x in 0..width{
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }

        // let newcolor = Color::new(colororder, iterations);

        ImgTools{
            height,
            width,
            path,
            img,
        }
    }

    pub fn update_image(&mut self, fractal_intensity: FractalIntensity){
        for i in 0..fractal_intensity.size{
            let [r, g, b, a] = fractal_intensity.intensity_color[i];
            self.img.put_pixel(
                fractal_intensity.screen_x[i], 
                fractal_intensity.screen_y[i], 
                // takes a [u8; 3] array and convert it into an Rgb type
                Rgba([r, g, b, a]),
            );
        }
    }

    // pub fn convertvalue(value: u32) -> [u8; 4]{
    //     convertvalue(value)
    // }

    pub fn save_image(&self){
        self.img.save(&self.path).unwrap();
    }

    pub fn create_window(&self){
        create_window(self.img.clone());
    }
}
