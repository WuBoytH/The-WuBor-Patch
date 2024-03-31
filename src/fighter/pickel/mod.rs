mod acmd;
mod status;
mod frame;

mod forge;
mod trolley;

pub fn install() {
    let agent = &mut smashline::Agent::new("pickel");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    forge::install();
    trolley::install();
}