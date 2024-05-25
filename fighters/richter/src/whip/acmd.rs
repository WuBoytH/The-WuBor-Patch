use super::*;

mod normals;
mod aerials;
mod smashes;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    aerials::install(agent);
    smashes::install(agent);
}