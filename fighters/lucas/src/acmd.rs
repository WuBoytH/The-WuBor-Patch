mod normals;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod lasso;
mod escape;
mod cliff;

pub fn install(agent: &mut smashline::Agent) {
    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    lasso::install(agent);
    escape::install(agent);
    cliff::install(agent);
}
