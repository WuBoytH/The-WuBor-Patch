mod acmd;
mod frame;
mod status;
mod agent_init;
mod fgc;

pub fn install() {
    let agent = &mut smashline::Agent::new("bayonetta");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    fgc::install();
    agent.install();
}