use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("mario_mantle");
    status::install(agent);
    agent.install();
}