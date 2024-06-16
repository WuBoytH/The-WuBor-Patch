use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("toonlink_boomerang");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}