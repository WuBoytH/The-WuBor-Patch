mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("mariod_drcapsule");
    acmd::install(agent);
    agent.install();
}