mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod helper;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
}