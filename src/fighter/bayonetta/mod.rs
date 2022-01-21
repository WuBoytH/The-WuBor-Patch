mod acmd;
mod frame;
mod status;
pub mod fgc;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}