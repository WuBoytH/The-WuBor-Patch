mod acmd;
mod status;
mod frame;
mod agent_init;
pub mod fgc;

pub fn install() {
    acmd::install();
    status::install();
    frame::install();
    agent_init::install();
}