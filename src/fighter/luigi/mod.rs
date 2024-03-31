mod acmd;
mod status;
mod frame;

mod fireball;

pub fn install() {
    let agent = &mut smashline::Agent::new("luigi");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    fireball::install();
}