use super::*;

mod escape;

mod special_hi_attack1;

mod special_lw;
mod special_lw_out;

pub fn install(agent: &mut smashline::Agent) {
    escape::install(agent);

    special_hi_attack1::install(agent);

    special_lw::install(agent);
    special_lw_out::install(agent);
}