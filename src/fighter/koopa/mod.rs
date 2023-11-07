mod acmd;
mod status;
mod frame;
mod vtable_hook;

mod breath;

pub fn install() {
    let agent = &mut smashline::Agent::new("koopa");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    vtable_hook::install();
    agent.install();

    breath::install();
}