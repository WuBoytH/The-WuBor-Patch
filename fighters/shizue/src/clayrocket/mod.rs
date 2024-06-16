use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("shizue_clayrocket");
    acmd::install(agent);
    agent.install();
}