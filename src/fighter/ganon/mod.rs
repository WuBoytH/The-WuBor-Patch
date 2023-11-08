mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod helper;
// mod vtable_hook;

pub fn install() {
    let agent = &mut smashline::Agent::new("ganon");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    // vtable_hook::install();
    agent.install();
}