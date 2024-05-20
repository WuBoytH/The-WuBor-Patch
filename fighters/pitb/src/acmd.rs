use super::*;

mod normals;
mod aerials;
mod specials;
mod appeal;
mod throws;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    appeal::install(agent);
    throws::install(agent);
    escape::install(agent);
    cliff::install(agent);
}