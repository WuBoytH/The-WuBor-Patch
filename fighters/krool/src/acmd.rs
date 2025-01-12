use super::*;

mod dash;

mod landing;

mod normals;
mod smashes;
mod aerials;

mod catch;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    landing::install(agent);

    normals::install(agent);
    smashes::install(agent);
    aerials::install(agent);

    catch::install(agent);

    escape::install(agent);
    cliff::install(agent);
}