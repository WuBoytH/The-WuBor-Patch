mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("shizue_bullet");
    acmd::install(agent);
    agent.install();
}