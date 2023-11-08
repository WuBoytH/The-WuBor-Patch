mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("toonlink_bowarrow");
    acmd::install(agent);
    agent.install();
}