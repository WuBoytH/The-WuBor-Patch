use super::*;

mod acmd;

mod vtable;

pub fn install() {
    let agent = &mut Agent::new("dolly_wave");
    acmd::install(agent);
    agent.install();

    vtable::install();
}