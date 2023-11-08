mod special_hi;
mod special_hi_rush;
mod special_hi_end;
mod special_hi_landing;
pub mod helper;

pub fn install(agent: &mut smashline::Agent) {
    special_hi::install(agent);
    special_hi_rush::install(agent);
    special_hi_end::install(agent);
    special_hi_landing::install(agent);
}