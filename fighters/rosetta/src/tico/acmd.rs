use super::*;

mod normals;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
}