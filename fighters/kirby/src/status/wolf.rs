use super::*;

mod special_n;
mod special_n_cancel;

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
    special_n_cancel::install(agent);
}