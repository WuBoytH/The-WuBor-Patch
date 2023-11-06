mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("palutena");
    acmd::install(agent);
    agent.install();
}