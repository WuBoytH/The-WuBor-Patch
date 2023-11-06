mod acmd;
mod status;

mod beam;

pub fn install() {
    let agent = &mut smashline::Agent::new("tantan");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    beam::install();
}