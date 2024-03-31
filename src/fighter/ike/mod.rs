mod acmd;
pub mod status;
mod frame;
mod agent_init;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("ike");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();
}