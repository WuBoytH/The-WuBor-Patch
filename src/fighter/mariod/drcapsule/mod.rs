mod acmd;
mod vtable_hook;

pub fn install() {
    let agent = &mut smashline::Agent::new("mariod_drcapsule");
    acmd::install(agent);
    vtable_hook::install();
    agent.install();
}