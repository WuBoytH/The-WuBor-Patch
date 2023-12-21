mod acmd;
mod frame;
mod status;
mod agent_init;
mod vtable_hook;
pub mod helper;
pub mod vl;
mod cancel;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucina");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    cancel::install();
    agent.install();
}