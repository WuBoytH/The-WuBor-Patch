mod normals;
mod specials;
mod throws;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    specials::install(agent);
    throws::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
