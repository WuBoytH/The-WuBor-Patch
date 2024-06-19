use super::*;

mod dash;

mod normals;
mod smashes;
mod aerials;
mod throws;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);

    escape::install(agent);
    cliff::install(agent);
}