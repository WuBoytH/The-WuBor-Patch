mod acmd;
mod status;
mod frame;
mod vtable_hook;

mod fireball;

pub fn install() {
    let agent = &mut smashline::Agent::new("luigi");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    vtable_hook::install();
    agent.install();

    fireball::install();
}