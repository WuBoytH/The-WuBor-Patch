mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("locario_qigong");
    acmd::install(agent);
    agent.install();
}