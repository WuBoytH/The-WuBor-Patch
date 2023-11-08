mod normals;
mod aerials;
mod specials;
mod escape;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    escape::install(agent);
    appeal::install(agent);
}