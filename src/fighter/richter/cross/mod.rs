mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("richter_cross");
    acmd::install(agent);
    agent.install();
}