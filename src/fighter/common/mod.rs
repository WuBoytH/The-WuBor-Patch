mod common_frame;
pub mod common_status;
mod agent_inits;
pub mod common_param;
pub mod command_inputs;
pub mod common_fgc;

pub fn install() {
    common_frame::install();
    common_status::install();
    agent_inits::install();
}