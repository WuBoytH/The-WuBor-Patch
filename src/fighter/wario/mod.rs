mod acmd;
mod status;
mod frame;
mod vtable_hook;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("wario");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    vtable_hook::install();
    agent.install();
}