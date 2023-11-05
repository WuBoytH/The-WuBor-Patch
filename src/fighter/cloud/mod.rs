mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("cloud");
    acmd::install(agent);
    agent.install();
}