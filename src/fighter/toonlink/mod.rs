mod acmd;
mod frame;
mod status;
pub mod vl;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}