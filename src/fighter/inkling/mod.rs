mod acmd;
mod status;
mod fgc;
pub mod helper;

pub fn install() {
    acmd::install();
    status::install();
    fgc::install();
}