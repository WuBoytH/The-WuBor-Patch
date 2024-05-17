use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("simon_whip");
    acmd::install(agent);
    agent.install();
}