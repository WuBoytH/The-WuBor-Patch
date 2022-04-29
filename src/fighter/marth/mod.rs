mod acmd;
mod frame;
mod status;
pub mod agent_init;
pub mod vars;
pub mod vl;
pub mod helper;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
}