mod acmd;
mod frame;
mod status;
mod agent_init;
mod vtable_hook;
pub mod helper;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    agent_init::install();
    vtable_hook::install();
}