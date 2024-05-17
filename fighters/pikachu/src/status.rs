use super::*;

mod special_s;
mod special_s_warp;
mod special_s_end;
mod special_hi;
mod special_lw;

pub fn install(agent: &mut smashline::Agent) {
    special_s::install(agent);
    special_s_warp::install(agent);
    special_s_end::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
}