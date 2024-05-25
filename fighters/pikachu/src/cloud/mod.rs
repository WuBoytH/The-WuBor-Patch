use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pikachu_cloud");
    acmd::install(agent);
    agent.install();
}