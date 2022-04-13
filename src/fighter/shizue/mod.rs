mod acmd;
mod frame;
pub mod agent_init;
pub mod helper;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
}