mod acmd;
mod status;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("donkey");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}