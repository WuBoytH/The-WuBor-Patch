use super::*;

mod special_n2;

pub fn install(agent: &mut Agent) {
    special_n2::install(agent);
}