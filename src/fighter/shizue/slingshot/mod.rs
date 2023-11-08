mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("shizue_slingshot");
    acmd::install(agent);
    agent.install();
}