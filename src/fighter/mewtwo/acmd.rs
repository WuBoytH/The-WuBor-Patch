mod aerials;
mod escape;
mod normals;
mod smashes;

pub fn install(agent: &mut smashline::Agent) {
    aerials::install(agent);
    escape::install(agent);
    normals::install(agent);
    smashes::install(agent);
}
