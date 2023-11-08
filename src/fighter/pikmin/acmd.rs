mod normals;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    escape::install(agent);
}