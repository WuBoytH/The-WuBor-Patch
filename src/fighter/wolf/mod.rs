mod acmd;
mod status;
mod frame;
mod fgc;

pub fn install() {
    let agent = &mut smashline::Agent::new("wolf");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    fgc::install();
    agent.install();
}