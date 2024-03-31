mod acmd;
mod status;
mod frame;
mod agent_init;

pub fn install() {
    let agent = &mut smashline::Agent::new("gaogaen");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();
}