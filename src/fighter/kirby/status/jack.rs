mod special_n;
mod special_n_escape;
mod special_n_jump;
pub mod helper;

pub fn install(agent : &mut smashline::Agent) {
    special_n::install(agent);
    special_n_escape::install(agent);
    special_n_jump::install(agent);
}