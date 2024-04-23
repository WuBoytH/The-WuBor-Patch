mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("snake_cypher");
    acmd::install(agent);
    agent.install();
}