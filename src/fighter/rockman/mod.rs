mod acmd;
mod status;
mod agent_init;
mod fgc;
mod vtable_hook;
pub mod vl;

mod chargeshot;
mod airshooter;
mod leafshield;

pub fn install() {
    let agent = &mut smashline::Agent::new("rockman");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    fgc::install();
    vtable_hook::install();
    agent.install();

    chargeshot::install();
    airshooter::install();
    leafshield::install();
}