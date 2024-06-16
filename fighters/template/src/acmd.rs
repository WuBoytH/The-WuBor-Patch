use super::*;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    escape::install(agent);
    cliff::install(agent);
}
