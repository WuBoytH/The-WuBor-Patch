mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("buddy");
    acmd::install(agent);
    agent.install();
}