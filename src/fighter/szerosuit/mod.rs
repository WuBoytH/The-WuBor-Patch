mod acmd;
mod status;
pub mod vars;

pub fn install() {
    acmd::install();
    status::install();
}