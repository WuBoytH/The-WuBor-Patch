use super::*;

mod dash;

mod normals;
mod smashes;
mod aerials;
mod specials;

mod catch;
mod throws;
mod lasso;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);

    catch::install(agent);
    throws::install(agent);
    lasso::install(agent);

    escape::install(agent);
    cliff::install(agent);
}
