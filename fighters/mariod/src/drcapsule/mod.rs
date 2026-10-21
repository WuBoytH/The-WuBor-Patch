use super::*;

mod acmd;

mod vtable;

pub fn install() {
    let agent = &mut Agent::new("mariod_drcapsule");
    acmd::install(agent);
    agent.install();

    vtable::install();
}