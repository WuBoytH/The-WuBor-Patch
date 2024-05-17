use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("rockman_leafshield");
    acmd::install(agent);
    agent.install();
}