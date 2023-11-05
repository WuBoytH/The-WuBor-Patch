mod acmd;
pub mod status;
mod agent_init;
mod vtable_hook;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("ike");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    agent.install();
}