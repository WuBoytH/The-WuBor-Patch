use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("samusd_supermissile");
    acmd::install(agent);
    agent.install();
}