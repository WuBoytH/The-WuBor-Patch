mod global_frames;
pub mod common_status;
mod agent_inits;
pub mod common_param;
pub mod command_inputs;

pub fn install() {
    global_frames::install();
    common_status::install();
    agent_inits::install();
}