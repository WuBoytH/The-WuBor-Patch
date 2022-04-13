mod acmd;
mod status;
pub mod fgc;
pub mod vl;
pub mod vars;

pub fn install() {
    acmd::install();
    status::install();
}