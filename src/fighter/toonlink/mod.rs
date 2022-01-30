mod acmd;
mod frame;
mod status;
pub mod fgc;
mod vl;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}