mod acmd;
mod frame;
mod agent_init;

pub fn install() {
    let agent = &mut smashline::Agent::new("peach");
    acmd::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();
}