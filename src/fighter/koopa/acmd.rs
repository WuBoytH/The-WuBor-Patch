mod normals;
mod specials;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    specials::install(agent);
    escape::install(agent);
}