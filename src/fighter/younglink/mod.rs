mod acmd;
mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("younglink");
    acmd::install(agent);
    frame::install(agent);
    agent.install();
}