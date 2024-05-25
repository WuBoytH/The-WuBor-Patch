use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("duckhunt_clay");
    acmd::install(agent);
    agent.install();
}