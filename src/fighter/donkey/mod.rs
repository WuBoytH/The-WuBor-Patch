mod acmd;
mod status;
mod frame;

pub fn install() {
    let agent = &mut smashline::Agent::new("donkey");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    smashline::add_param_object("donkey", "param_barrel");
    agent.install();
}