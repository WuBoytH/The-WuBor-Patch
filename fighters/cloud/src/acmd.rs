use super::*;

mod normals;
mod aerials;
mod specials;
mod throws;
mod escape;
mod cliff;
mod appeal;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    escape::install(agent);
    cliff::install(agent);
    appeal::install(agent);
}