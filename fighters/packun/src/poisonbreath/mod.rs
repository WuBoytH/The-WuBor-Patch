use super::*;

mod acmd;
mod status;

mod vtable;

pub fn install() {
    let agent = &mut smashline::Agent::new("packun_poisonbreath");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    vtable::install();
}