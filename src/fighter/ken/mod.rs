mod acmd;
mod status;
mod frame;
mod agent_init;

mod hadoken;

pub fn install() {
    let agent = &mut smashline::Agent::new("ken");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();

    hadoken::install();
}