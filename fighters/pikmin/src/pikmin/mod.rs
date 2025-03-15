use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("pikmin_pikmin");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}