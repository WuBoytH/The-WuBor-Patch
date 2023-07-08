mod acmd;
mod frame;
mod status;
mod vtable_hook;
pub mod fgc;
pub mod agent_init;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    vtable_hook::install();
    agent_init::install();
}