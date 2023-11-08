mod normals;
mod aerials;
mod specials;
mod throws;
mod escape;
mod appeal;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    escape::install(agent);
    appeal::install(agent);
}