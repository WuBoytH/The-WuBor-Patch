use super::*;

mod special_hi_jump;
mod special_lw;
mod special_lw_select;
mod special_lw_start;
mod special_lw_crash_end;
pub mod helper;

pub fn install(agent: &mut Agent) {
    special_hi_jump::install(agent);
    special_lw::install(agent);
    special_lw_select::install(agent);
    special_lw_start::install(agent);
    special_lw_crash_end::install(agent);
}