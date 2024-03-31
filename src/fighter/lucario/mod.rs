mod acmd;
mod status;
mod frame;
mod agent_init;
pub mod helper;
mod fgc;

mod auraball;
mod qigong;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucario");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    fgc::install();
    smashline::add_param_object("lucario", "param_auracharge");
    agent.install();

    auraball::install();
    qigong::install();
}