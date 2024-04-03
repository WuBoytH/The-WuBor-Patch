mod acmd;
mod status;
mod frame;

mod c4;

pub fn install() {
    let agent = &mut smashline::Agent::new("snake");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    c4::install();
}