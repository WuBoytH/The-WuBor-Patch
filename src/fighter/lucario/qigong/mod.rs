mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucario_qigong");
    acmd::install(agent);
    agent.install();
}