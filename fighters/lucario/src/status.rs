use super::*;

mod special_n;

mod special_s;

mod special_hi;

mod special_lw;

mod escape_air;

mod escape_air_slide;

mod landing;

pub fn install(agent: &mut smashline::Agent) {
    special_n::install(agent);

    special_s::install(agent);

    special_hi::install(agent);

    special_lw::install(agent);

    escape_air::install(agent);

    escape_air_slide::install(agent);

    landing::install(agent);
}