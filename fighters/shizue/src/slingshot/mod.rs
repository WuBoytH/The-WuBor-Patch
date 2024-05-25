use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("shizue_slingshot");
    acmd::install(agent);
    agent.install();
}