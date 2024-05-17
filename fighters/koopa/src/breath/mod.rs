use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut smashline::Agent::new("koopa_breath");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}