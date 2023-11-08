mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("gamewatch_breath");
    acmd::install(agent);
    agent.install();
}