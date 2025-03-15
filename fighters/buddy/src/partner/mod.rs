use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("buddy_partner");
    acmd::install(agent);
    agent.install();
}