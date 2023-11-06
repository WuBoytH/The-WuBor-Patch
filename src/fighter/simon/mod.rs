mod acmd;
mod status;
mod agent_init;

pub fn install() {
    let agent = &mut smashline::Agent::new("simon");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();
}