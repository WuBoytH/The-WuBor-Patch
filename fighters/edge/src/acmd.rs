use super::*;

mod normals;
mod aerials;
mod throws;
mod specials;
mod escape;
mod cliff;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    throws::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
    appeal::install(agent);
}