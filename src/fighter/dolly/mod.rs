mod acmd;
mod frame;
mod status;
pub mod agent_init;
pub mod helper;
pub mod vtable_hook;

// mod wave;
// mod burst;

pub fn install() {
    let agent = &mut smashline::Agent::new("dolly");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    smashline::add_param_object("dolly", "param_misc");
    agent.install();

    // wave::install();
    // burst::install();
}