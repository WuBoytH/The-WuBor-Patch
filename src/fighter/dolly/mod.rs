mod acmd;
mod frame;
mod status;
pub mod agent_init;
pub mod helper;
pub mod vl;
pub mod vtable_hook;

pub fn install() {
    let agent = &mut smashline::Agent::new("dolly");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    agent.install();
}