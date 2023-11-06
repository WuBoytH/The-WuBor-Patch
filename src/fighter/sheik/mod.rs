mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("sheik");
    acmd::install(agent);
    agent.install();
}