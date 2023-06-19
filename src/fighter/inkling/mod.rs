mod acmd;
mod status;
mod agent_init;
pub mod fgc;
pub mod helper;

pub fn install() {
    acmd::install();
    status::install();
    agent_init::install();
}