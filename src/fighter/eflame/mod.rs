mod acmd;
mod status;
mod frame;
mod agent_init;

mod esword;

pub fn install() {
    let agent = &mut smashline::Agent::new("eflame");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();

    esword::install();
}