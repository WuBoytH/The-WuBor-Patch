use super::*;

mod normals;
mod aerials;
mod smashes;

mod throws;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    aerials::install(agent);
    smashes::install(agent);

    throws::install(agent);
}