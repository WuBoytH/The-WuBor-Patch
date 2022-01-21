mod acmd;
mod frame;
pub mod fgc;

pub fn install() {
    acmd::install();
    frame::install();
}