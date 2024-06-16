use super::*;

mod special_hi_warp;

pub fn install(agent: &mut Agent) {
    special_hi_warp::install(agent);
}