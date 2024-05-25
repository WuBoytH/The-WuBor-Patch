use super::*;

mod normals;
mod aerials;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
    aerials::install(agent);
}