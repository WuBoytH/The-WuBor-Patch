mod special_n;
mod special_n_escape;
mod special_n_jump;
pub mod special_s;
pub mod special_lw;
mod summon;
mod dispatch;
pub mod helper;

pub fn install(agent: &mut smashline::Agent) {
    special_n::install(agent);
    special_n_escape::install(agent);
    special_n_jump::install(agent);
    special_s::install(agent);
    summon::install(agent);
    dispatch::install(agent);
}