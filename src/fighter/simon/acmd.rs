mod normals;
mod smashes;
mod aerials;
mod throws;
mod specials;
mod escape;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    throws::install(agent);
    specials::install(agent);
    escape::install(agent);
}
