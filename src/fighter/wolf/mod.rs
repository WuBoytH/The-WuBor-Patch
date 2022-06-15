mod acmd;
mod agent_init;
pub mod fgc;

pub fn install() {
    acmd::install();
    agent_init::install();
}