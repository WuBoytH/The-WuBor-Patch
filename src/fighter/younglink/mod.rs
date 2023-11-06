mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("younglink");
    acmd::install(agent);
    agent.install();
}