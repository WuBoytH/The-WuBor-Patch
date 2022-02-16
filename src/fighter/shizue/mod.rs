mod acmd;
mod frame;
pub mod agent_init;
pub mod helper;

pub fn install() {
    acmd::install();
    frame::install();
}