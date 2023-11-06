mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("roy");
    acmd::install(agent);
    agent.install();
}