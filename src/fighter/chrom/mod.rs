mod acmd;
mod status;
mod frame;
mod fgc;

pub fn install() {
    let agent = &mut smashline::Agent::new("chrom");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    fgc::install();
    smashline::add_param_object("chrom", "param_quake_slash");
    agent.install();
}