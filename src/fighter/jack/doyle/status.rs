mod entry;
pub mod helper;

pub fn install(agent : &mut smashline::Agent) {
    entry::install(agent);
}