mod acmd;
mod status;
mod frame;
mod vtable_hook;

pub fn install() {
    let agent = &mut smashline::Agent::new("elight");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    vtable_hook::install();
    agent.install();
}