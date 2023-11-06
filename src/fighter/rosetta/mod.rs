mod acmd;

mod tico;

pub fn install() {
    let agent = &mut smashline::Agent::new("rosetta");
    acmd::install(agent);
    agent.install();

    tico::install();
}