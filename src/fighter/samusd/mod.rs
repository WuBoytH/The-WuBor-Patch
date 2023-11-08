mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod vl;

mod cshot;
mod missile;
mod supermissile;

pub fn install() {
    let agent = &mut smashline::Agent::new("samusd");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    cshot::install();
    missile::install();
    supermissile::install();
}