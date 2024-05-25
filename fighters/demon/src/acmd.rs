use super::*;

mod jabs;
mod normals;
mod smashes;
mod catch;
mod throws;
mod aerials;
mod specials;
mod stand;
mod step;
mod squat;
mod escape;
mod cliff;
mod appeal;

pub fn install(agent: &mut Agent) {
    jabs::install(agent);
    normals::install(agent);
    smashes::install(agent);
    catch::install(agent);
    throws::install(agent);
    aerials::install(agent);
    specials::install(agent);
    stand::install(agent);
    step::install(agent);
    squat::install(agent);
    escape::install(agent);
    cliff::install(agent);
    appeal::install(agent);
}