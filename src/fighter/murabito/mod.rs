mod acmd;
mod status;
pub mod helper;

pub fn install() {
    let agent = &mut smashline::Agent::new("murabito");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}