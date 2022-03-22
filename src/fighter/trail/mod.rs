mod acmd;
mod status;
pub mod agent_init;
pub mod fgc;

pub fn install() {
    acmd::install();
    status::install();
}