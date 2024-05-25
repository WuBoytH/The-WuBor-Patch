use super::*;

mod special_hi;
mod rebirth;

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
    rebirth::install(agent);
}