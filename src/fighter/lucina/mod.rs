mod acmd;
mod frame;
mod status;
pub mod agent_init;
pub mod helper;
pub mod vl;
pub mod vars;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}