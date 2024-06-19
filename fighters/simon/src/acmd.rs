use super::*;

mod dash;

mod normals;
mod smashes;
mod aerials;
mod specials;
mod throws;

mod escape;
mod cliff;

mod appeal;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);

    appeal::install(agent); 
}
