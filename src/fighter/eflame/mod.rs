mod acmd;
mod status;
mod agent_init;
mod vtable_hook;

mod esword;

pub fn install() {
    let agent = &mut smashline::Agent::new("eflame");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    vtable_hook::install();
    agent.install();

    esword::install();
}