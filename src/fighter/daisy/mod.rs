mod acmd;
mod frame;
mod status;
pub mod agent_init;
mod vl;
pub mod fgc;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    agent_init::install();
}