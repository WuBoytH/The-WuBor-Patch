mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("duckhunt_clay");
    acmd::install(agent);
    agent.install();
}