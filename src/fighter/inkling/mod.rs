mod acmd;
mod status;
mod frame;
mod fgc;
pub mod helper;

pub fn install() {
    let agent = &mut smashline::Agent::new("inkling");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    fgc::install();
    agent.install();
}