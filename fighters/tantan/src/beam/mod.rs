use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("tantan_beam");
    acmd::install(agent);
    agent.install();
}