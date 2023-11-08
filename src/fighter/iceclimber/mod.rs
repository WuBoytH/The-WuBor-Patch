mod acmd;
mod frame;

pub fn install() {
    let popo = &mut smashline::Agent::new("popo");
    let nana = &mut smashline::Agent::new("nana");
    acmd::install(popo);
    acmd::install(nana);
    frame::install(popo);
    frame::install(nana);
    popo.install();
    nana.install();
}