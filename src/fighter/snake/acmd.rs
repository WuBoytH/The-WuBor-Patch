// mod normals;
// mod aerials;
mod specials;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    // normals::install();
    // aerials::install();
    specials::install(agent);
    escape::install(agent);
}