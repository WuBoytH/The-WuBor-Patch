mod acmd;
mod frame;
mod agent_init;

pub fn install() {
    acmd::install();
    frame::install();
    agent_init::install();
}