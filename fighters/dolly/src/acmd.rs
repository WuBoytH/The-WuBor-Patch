use super::*;

mod normals;
mod smashes;
mod catch;
mod throws;
mod aerials;
mod specials;
mod escape;
mod cliff;
mod appeal;
mod misc;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    smashes::install(agent);
    catch::install(agent);
    throws::install(agent);
    aerials::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
    appeal::install(agent);
    misc::install(agent);
}