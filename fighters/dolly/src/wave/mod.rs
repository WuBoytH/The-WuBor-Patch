use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("dolly_wave");
    acmd::install(agent);
    agent.install();
}