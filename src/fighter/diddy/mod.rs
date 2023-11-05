mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("diddy");
    acmd::install(agent);
    agent.install();
}