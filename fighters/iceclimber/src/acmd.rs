use super::*;

mod dash;

mod normals;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);

    escape::install(agent);
    cliff::install(agent);
}
