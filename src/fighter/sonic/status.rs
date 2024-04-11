mod jump;
mod throw;

mod trick;

pub fn install(agent: &mut smashline::Agent) {
    jump::install(agent);
    throw::install(agent);

    trick::install(agent);
}