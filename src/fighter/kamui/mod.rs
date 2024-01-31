mod acmd;
mod frame;
mod status;
mod agent_init;

mod waterdragon;

pub fn install() {
    let agent = &mut smashline::Agent::new("kamui");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    waterdragon::install();
}