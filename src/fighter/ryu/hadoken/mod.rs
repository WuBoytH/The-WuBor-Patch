mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("ryu_hadoken");
    acmd::install(agent);
    agent.install();
}