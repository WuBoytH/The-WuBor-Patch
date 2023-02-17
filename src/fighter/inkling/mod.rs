mod acmd;
mod agent_init;
pub mod fgc;
pub mod helper;

pub fn install() {
    acmd::install();
    agent_init::install();
}