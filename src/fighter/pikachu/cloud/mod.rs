mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pikachu_cloud");
    acmd::install(agent);
    agent.install();
}