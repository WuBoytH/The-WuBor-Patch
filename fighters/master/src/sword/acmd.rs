use super::*;

mod normals;

pub fn install(agent: &mut Agent) {
    normals::install(agent);
}