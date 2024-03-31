mod acmd;
mod status;
mod frame;
mod agent_init;

mod whip;
mod axe;
mod cross;

pub fn install() {
    let agent = &mut smashline::Agent::new("richter");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();

    whip::install();
    axe::install();
    cross::install();
}