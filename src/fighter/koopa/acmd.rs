mod normals;
mod specials;
mod escape;
mod cliff;
mod aerials;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
    aerials::install(agent);
}