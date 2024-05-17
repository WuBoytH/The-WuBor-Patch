use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("richter_whip");
    acmd::install(agent);
    agent.install();
}