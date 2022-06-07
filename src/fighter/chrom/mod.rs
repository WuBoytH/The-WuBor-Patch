mod acmd;
mod status;
mod agent_init;
pub mod fgc;
pub mod vl;

pub fn install() {
    acmd::install();
    status::install();
    agent_init::install();
}