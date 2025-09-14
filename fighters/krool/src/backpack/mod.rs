use super::*;

mod acmd;
mod frame;

pub fn install() {
    let agent = &mut Agent::new("krool_backpack");
    acmd::install(agent);
    frame::install(agent);
    agent.install();
}