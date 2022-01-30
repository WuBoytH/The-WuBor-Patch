mod acmd;
mod frame;
pub mod agent_init;
pub mod fgc;

pub fn install() {
    acmd::install();
    frame::install();
}