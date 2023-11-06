mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("template");
    acmd::install(agent);
    agent.install();
}