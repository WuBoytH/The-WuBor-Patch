mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pzenigame");
    acmd::install(agent);
    agent.install();
}