
/// Returns a RGBA value based on the proportion of value to max iteration.
pub fn convertvalue(value: u32) -> [u8; 4]{
    let proportion: f64 = value as f64/(crate::libs::globals::ITERATIONS as f64);
    // x increments from 0 to PI/2
    // Not sure why adding use statement not including in this context
    let pi = std::f64::consts::PI;
    let x: f64 = proportion * pi/2.0;
    let (r, g, b, a) = (
        ((x*7.0).sin()).abs() * 255.0,
        ((x*3.0).sin()).abs() * 255.0, 
        ((x*1.0).sin()).abs() * 255.0,
        255.0,
    );
    [r as u8, g as u8, b as u8, a as u8]
}