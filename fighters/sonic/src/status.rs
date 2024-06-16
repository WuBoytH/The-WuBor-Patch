use super::*;

mod jump;
mod throw;

pub fn install(agent: &mut Agent) {
    jump::install(agent);
    throw::install(agent);
}