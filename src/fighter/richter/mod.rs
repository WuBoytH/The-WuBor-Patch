mod acmd;
mod frame;
mod agent_init;
pub mod fgc;

pub fn install() {
    acmd::install();
    frame::install();
    agent_init::install();
}