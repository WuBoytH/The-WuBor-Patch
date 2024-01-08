mod acmd;
mod frame;
mod status;
pub mod agent_init;

pub fn install() {
    let agent = &mut smashline::Agent::new("daisy");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    smashline::add_param_object("daisy", "param_parasol_plummet");
    agent.install();
}