use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("rosetta_tico");
    acmd::install(agent);
    agent.install();
}