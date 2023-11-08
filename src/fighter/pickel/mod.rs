mod acmd;
mod status;
mod frame;
mod vtable_hook;

mod forge;
mod trolley;

pub fn install() {
    let agent = &mut smashline::Agent::new("pickel");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    vtable_hook::install();
    agent.install();

    forge::install();
    trolley::install();
}