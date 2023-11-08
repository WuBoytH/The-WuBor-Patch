mod acmd;
mod status;
mod frame;

mod sword;

pub fn install() {
    let agent = &mut smashline::Agent::new("master");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    sword::install();
}