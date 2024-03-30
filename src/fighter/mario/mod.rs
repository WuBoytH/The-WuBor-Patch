mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod helper;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    wubor_utils::wua_bind::MiscModule::patch_vtable_function(0x51e4630, smash::hash40("trans"));
    wubor_utils::wua_bind::MiscModule::patch_vtable_function(0x51e4638, smash::hash40("trans"));
}