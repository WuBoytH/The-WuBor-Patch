mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("littlemac");
    acmd::install(agent);
    agent.install();
}