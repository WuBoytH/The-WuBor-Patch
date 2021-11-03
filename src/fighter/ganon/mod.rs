mod acmd;
mod frame;
mod status;
pub mod helper;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}