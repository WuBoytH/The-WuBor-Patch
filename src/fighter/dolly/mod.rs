mod acmd;
mod frame;
mod status;
pub mod agent_init;
pub mod fgc;
pub mod helper;
pub mod vl;
pub mod vtable_hook;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    agent_init::install();
    vtable_hook::install();
}