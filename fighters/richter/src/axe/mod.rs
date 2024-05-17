use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("richter_axe");
    acmd::install(agent);
    agent.install();
}