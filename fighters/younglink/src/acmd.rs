use super::*;

mod dash;

mod guard;

mod normals;
mod specials;

mod catch;
mod lasso;

mod escape;
mod cliff;
mod appeal;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    guard::install(agent);

    normals::install(agent);
    specials::install(agent);

    catch::install(agent);
    lasso::install(agent);

    escape::install(agent);
    cliff::install(agent);
    appeal::install(agent);
}
