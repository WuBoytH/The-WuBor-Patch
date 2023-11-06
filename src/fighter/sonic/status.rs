mod jump;
mod throw;

pub fn install(agent : &mut smashline::Agent) {
    jump::install(agent);
    throw::install(agent);
}