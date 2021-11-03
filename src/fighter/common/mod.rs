mod global_frames;
pub mod common_status;
mod agent_inits;

pub fn install() {
    global_frames::install();
    common_status::install();
    agent_inits::install();
}