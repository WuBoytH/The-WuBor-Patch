mod acmd;
mod status;
mod vtable_hook;

pub fn install() {
    let agent = &mut smashline::Agent::new("luigi");
    acmd::install(agent);
    status::install(agent);
    vtable_hook::install();
    agent.install();
}