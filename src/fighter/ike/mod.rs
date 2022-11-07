mod acmd;
pub mod status;
mod agent_init;
mod vtable_hook;
pub mod vl;

pub fn install() {
    acmd::install();
    status::install();
    agent_init::install();
    vtable_hook::install();
}