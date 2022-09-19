mod acmd;
mod agent_init;
pub mod fgc;
mod frame;

pub fn install() {
    acmd::install();
    agent_init::install();
    frame::install();
}