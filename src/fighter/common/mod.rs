pub mod frame;
pub mod status;
pub mod agent_inits;
pub mod param;
// pub mod command_inputs;
mod energy;

pub fn install() {
    status::install();
    energy::install();
}