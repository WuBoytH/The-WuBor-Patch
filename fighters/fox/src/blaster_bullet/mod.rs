use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("fox_blaster_bullet");
    acmd::install(agent);
    agent.install();
}