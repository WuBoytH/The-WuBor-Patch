use super::*;

mod dash;

mod normals;
mod aerials;
mod specials;
mod throws;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);
}