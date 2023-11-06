mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("zelda");
    acmd::install(agent);
    agent.install();
}