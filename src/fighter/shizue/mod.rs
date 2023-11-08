mod acmd;
mod status;
mod frame;
mod agent_init;
pub mod helper;

mod slingshot;
mod bullet;
mod clayrocket;

pub fn install() {
    let agent = &mut smashline::Agent::new("shizue");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();
    
    slingshot::install();
    bullet::install();
    clayrocket::install();
}