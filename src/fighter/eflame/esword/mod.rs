mod acmd;
mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("eflame_esword");
    acmd::install(agent);
    frame::install(agent);
    agent.install();
}