mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("shizue_clayrocket");
    acmd::install(agent);
    agent.install();
}