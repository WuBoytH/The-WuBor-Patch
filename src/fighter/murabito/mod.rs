mod acmd;
mod status;
mod frame;
pub mod helper;

pub fn install() {
    let agent = &mut smashline::Agent::new("murabito");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();
}