use super::*;

mod acmd;
mod status;

mod vtable;

pub fn install() {
    let agent = &mut Agent::new("fox_blaster_bullet");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    vtable::install();
}