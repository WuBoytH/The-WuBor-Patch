// mod normals;
// mod aerials;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    // normals::install();
    // aerials::install();
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}