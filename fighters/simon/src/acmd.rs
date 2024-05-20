use super::*;

mod normals;
mod smashes;
mod aerials;
mod throws;
mod specials;
mod appeal;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);
    specials::install(agent);
    appeal::install(agent); 
    escape::install(agent);
    cliff::install(agent);
}
