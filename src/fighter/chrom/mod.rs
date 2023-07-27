mod acmd;
mod status;
mod fgc;
pub mod vl;

pub fn install() {
    acmd::install();
    status::install();
    fgc::install();
}