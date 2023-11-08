mod acmd;
mod status;
mod frame;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("donkey");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();
}