mod acmd;
mod status;
pub mod helper;

pub fn install() {
    acmd::install();
    status::install();
}