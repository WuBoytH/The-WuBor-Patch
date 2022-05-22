mod acmd;
mod frame;
mod agent_init;
pub mod fgc;
pub mod vars;
mod vtable_hook;

pub fn install() {
    acmd::install();
    frame::install();
    agent_init::install();
    vtable_hook::install();
}