mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod helper;
// mod vtable_hook;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    agent_init::install();
    // vtable_hook::install();
}