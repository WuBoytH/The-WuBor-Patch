mod acmd;
mod status;
mod fgc;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("chrom");
    acmd::install(agent);
    status::install(agent);
    fgc::install();
    agent.install();
}