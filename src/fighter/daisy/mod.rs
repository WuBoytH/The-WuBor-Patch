mod acmd;
mod frame;
mod status;
pub mod agent_init;
mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("daisy");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();
}