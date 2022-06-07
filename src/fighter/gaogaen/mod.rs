mod acmd;
mod frame;
pub mod helper;

pub fn install() {
    acmd::install();
    frame::install();
}