mod acmd;
mod status;
mod agent_init;
mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("sonic");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    frame::install(agent);
    agent.install();
}