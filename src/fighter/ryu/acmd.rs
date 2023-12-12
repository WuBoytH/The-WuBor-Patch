mod normals;
mod smashes;
mod aerials;
mod specials;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    escape::install(agent);
    cliff::install(agent);
}