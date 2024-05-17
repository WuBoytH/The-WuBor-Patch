use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("samusd_cshot");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}