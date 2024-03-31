mod acmd;
mod frame;
mod status;
pub mod agent_init;
pub mod helper;

// mod wave;
// mod burst;

pub fn install() {
    let agent = &mut smashline::Agent::new("dolly");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    smashline::add_param_object("dolly", "param_misc");
    agent.install();

    // wave::install();
    // burst::install();
}