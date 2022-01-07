mod acmd;
mod status;
pub mod vl;

pub fn install() {
    acmd::install();
    status::install();
}