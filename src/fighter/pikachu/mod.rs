mod acmd;
mod status;
mod frame;
mod agent_init;
pub mod vl;

mod dengekidama;
mod cloud;
mod kaminari;

pub fn install() {
    let agent = &mut smashline::Agent::new("pikachu");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();

    dengekidama::install();
    cloud::install();
    kaminari::install();
}