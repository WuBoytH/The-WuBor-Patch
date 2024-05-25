use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("dolly_burst");
    acmd::install(agent);
    agent.install();
}