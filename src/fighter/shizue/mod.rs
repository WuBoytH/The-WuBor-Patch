mod acmd;
mod frame;
mod agent_init;
pub mod helper;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
    agent_init::install();
}