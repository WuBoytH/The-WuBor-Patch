use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("master_sword");
    acmd::install(agent);
    agent.install();
}