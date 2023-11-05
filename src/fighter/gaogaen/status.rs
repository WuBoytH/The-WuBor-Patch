mod special_lw;
pub mod helper;

pub fn install(agent : &mut smashline::Agent) {
    special_lw::install(agent);
}