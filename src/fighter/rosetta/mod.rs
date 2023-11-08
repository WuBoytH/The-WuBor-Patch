mod acmd;
mod frame;

mod tico;

pub fn install() {
    let agent = &mut smashline::Agent::new("rosetta");
    acmd::install(agent);
    frame::install(agent);
    agent.install();

    tico::install();
}