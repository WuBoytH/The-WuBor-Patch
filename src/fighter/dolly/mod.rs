mod acmd;
mod status;
pub mod agent_init;
pub mod helper;

pub fn install() {
    acmd::install();
    status::install();
}