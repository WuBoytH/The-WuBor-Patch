mod acmd;
mod frame;
mod status;
mod agent_init;
// mod vtable_hook;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    agent_init::install();
    // vtable_hook::install();
}