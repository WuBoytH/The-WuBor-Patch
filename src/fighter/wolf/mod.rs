mod acmd;
mod status;
mod agent_init;

pub fn install() {
    acmd::install();
    status::install();
    agent_init::install();
}