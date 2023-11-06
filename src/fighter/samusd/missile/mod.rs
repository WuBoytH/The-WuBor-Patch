mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("samusd_missile");
    acmd::install(agent);
    agent.install();
}