mod normals;
mod aerials;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    escape::install(agent);
}