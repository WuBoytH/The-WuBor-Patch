use super::*;

mod dash;

mod normals;
mod specials;

mod catch;
mod lasso;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    normals::install(agent);
    specials::install(agent);

    catch::install(agent);
    lasso::install(agent);

    escape::install(agent);
    cliff::install(agent);
}
