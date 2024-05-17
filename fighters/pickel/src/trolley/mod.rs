use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("pickel_trolley");
    acmd::install(agent);
    agent.install();
}