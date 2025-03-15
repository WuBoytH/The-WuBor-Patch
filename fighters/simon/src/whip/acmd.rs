use super::*;

mod guard;

mod normals;
mod aerials;

pub fn install(agent: &mut Agent) {
    guard::install(agent);

    normals::install(agent);
    aerials::install(agent);
}