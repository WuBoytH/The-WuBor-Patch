mod acmd;
mod status;
mod frame;

mod drcapsule;

pub fn install() {
    let agent = &mut smashline::Agent::new("mariod");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    drcapsule::install();
}