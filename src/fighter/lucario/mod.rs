mod acmd;
mod status;
mod frame;
mod agent_init;
pub mod helper;
mod vtable_hook;
mod fgc;
pub mod vl;

mod auraball;
mod qigong;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucario");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    fgc::install();
    agent.install();

    auraball::install();
    qigong::install();
}