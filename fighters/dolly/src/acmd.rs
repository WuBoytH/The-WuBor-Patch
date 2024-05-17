use super::*;

mod normals;
mod smashes;
mod catch;
mod throws;
mod aerials;
mod specials;
mod escape;
mod cliff;
mod misc;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    smashes::install(agent);
    catch::install(agent);
    throws::install(agent);
    aerials::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
    misc::install(agent);
}