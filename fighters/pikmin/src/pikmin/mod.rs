use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pikmin_pikmin");
    acmd::install(agent);
    agent.install();
}