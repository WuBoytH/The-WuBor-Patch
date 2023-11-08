mod acmd;
mod frame;
// pub mod fgc;

pub fn install() {
    let agent = &mut smashline::Agent::new("miifighter");
    acmd::install(agent);
    frame::install(agent);
    agent.install();
}