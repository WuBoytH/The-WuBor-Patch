mod acmd;
mod status;
mod agent_init;
mod fgc;
mod vtable_hook;
pub mod vl;

pub fn install() {
    acmd::install();
    status::install();
    agent_init::install();
    fgc::install();
    vtable_hook::install();
}