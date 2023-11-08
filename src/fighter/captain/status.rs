mod special_s;
pub mod helper;

pub fn install(agent: &mut smashline::Agent) {
    special_s::install(agent);
}