mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("link");
    acmd::install(agent);
    agent.install();
}