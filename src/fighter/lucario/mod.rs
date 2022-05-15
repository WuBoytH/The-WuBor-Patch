mod acmd;
mod frame;
pub mod agent_init;
pub mod fgc;
pub mod vars;
mod vtable_hook;

pub fn install() {
    acmd::install();
    frame::install();
    vtable_hook::install();
}