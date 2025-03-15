use super::*;

mod guard;

mod normals;

pub fn install(agent: &mut Agent) {
    guard::install(agent);

    normals::install(agent);
}