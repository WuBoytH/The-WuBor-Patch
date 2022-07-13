mod acmd;
mod agent_init;

pub fn install() {
    acmd::install();
    agent_init::install();
}