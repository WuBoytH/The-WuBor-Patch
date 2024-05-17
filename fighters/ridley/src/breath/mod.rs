mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("ridley_breath");
    status::install(agent);
    agent.install();
}