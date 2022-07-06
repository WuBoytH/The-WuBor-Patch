mod acmd;
mod status;
mod frame;
mod agent_init;
pub mod fgc;
pub mod helper;
mod vtable_hook;
pub mod vl;

pub fn install() {
    acmd::install();
    status::install();
    frame::install();
    agent_init::install();
    vtable_hook::install();
}