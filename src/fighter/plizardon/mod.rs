mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("plizardon");
    acmd::install(agent);
    agent.install();
}