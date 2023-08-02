mod frame;
pub mod status;
mod agent_status;
pub mod agent_inits;
pub mod param;
// pub mod command_inputs;
mod vtable_hook;
mod energy;

pub fn install() {
    frame::install();
    status::install();
    agent_status::install();
    vtable_hook::install();
    energy::install();
}