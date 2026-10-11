use super::*;

mod acmd;
mod status;

mod vtable;

pub fn install() {
    let agent = &mut Agent::new("koopa_breath");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    vtable::install();
}