mod acmd;
mod status;
mod frame;
mod agent_init;
mod vtable_hook;

pub fn install() {
    let agent = &mut smashline::Agent::new("reflet");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    agent.install();
}