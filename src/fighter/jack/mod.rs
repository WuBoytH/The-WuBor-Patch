mod acmd;
mod frame;
mod status;
pub mod agent_init;
mod fgc;

mod doyle;

pub fn install() {
    let agent = &mut smashline::Agent::new("jack");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    fgc::install();
    agent.install();

    doyle::install();
}