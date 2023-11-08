mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("simon_axe");
    acmd::install(agent);
    agent.install();
}