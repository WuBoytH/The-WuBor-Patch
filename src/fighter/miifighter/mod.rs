mod acmd;
// pub mod fgc;

pub fn install() {
    let agent = &mut smashline::Agent::new("miifighter");
    acmd::install(agent);
    agent.install();
}