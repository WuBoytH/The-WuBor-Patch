mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucas_pkfire");
    acmd::install(agent);
    agent.install();
}