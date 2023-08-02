mod acmd;
mod frame;
mod status;
mod agent_init;
mod fgc;

pub fn install() {
    acmd::install();
    frame::install();
    status::install();
    agent_init::install();
    fgc::install();
}