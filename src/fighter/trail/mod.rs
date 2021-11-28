mod acmd;
mod status;
pub mod agent_init;

pub fn install() {
    acmd::install();
    status::install();
}