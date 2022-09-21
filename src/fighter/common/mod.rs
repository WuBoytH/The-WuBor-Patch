mod common_frame;
pub mod common_status;
mod agent_status;
pub mod agent_inits;
pub mod common_param;
// pub mod command_inputs;
pub mod common_fgc;
mod common_vtable;
mod vars;
// mod energy;

pub fn install() {
    common_frame::install();
    common_status::install();
    // agent_inits::install();
    agent_status::install();
    common_fgc::install();
    common_vtable::install();
    vars::install();
    // energy::install();
}