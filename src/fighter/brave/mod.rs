mod acmd;
mod status;
mod frame;
mod agent_init;
mod vtable_hook;

pub fn install() {
    acmd::install();
    status::install();
    frame::install();
    agent_init::install();
    vtable_hook::install();
}