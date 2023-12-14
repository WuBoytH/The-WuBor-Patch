mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("eflame_firepillar");
    acmd::install(agent);
    agent.install();
}