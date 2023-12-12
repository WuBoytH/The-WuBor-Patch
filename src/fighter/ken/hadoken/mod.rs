mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("ken_hadoken");
    acmd::install(agent);
    agent.install();
}