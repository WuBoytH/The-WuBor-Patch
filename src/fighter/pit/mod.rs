mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pit");
    acmd::install(agent);
    agent.install();
}