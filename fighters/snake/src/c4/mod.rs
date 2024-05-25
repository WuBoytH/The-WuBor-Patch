use super::*;

mod acmd;
mod frame;

pub fn install() {
    let agent = &mut Agent::new("snake_c4");
    acmd::install(agent);
    frame::install(agent);
    agent.install();
}