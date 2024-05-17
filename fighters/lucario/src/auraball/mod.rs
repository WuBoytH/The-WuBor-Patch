use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucario_auraball");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}