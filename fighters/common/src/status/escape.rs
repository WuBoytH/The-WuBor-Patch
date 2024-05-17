use super::*;

mod escape;
mod escape_air;
pub mod escape_air_slide;

pub fn install() {
    escape::install();
    escape_air::install();
    escape_air_slide::install();
}