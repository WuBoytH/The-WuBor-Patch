mod acmd;
mod frame;
pub mod helper;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
}