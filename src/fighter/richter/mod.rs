mod acmd;
mod status;
mod frame;
mod agent_init;
pub mod fgc;
mod vtable_hook;
pub mod helper;

pub fn install() {
    acmd::install();
    status::install();
    frame::install();
    agent_init::install();
    vtable_hook::install();
}