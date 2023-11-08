mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("master_sword");
    acmd::install(agent);
    agent.install();
}