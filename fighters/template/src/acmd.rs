use super::*;

mod dash;

mod escape;
mod cliff;

pub fn install(agent: &mut Agent) {
    dash::install(agent);

    escape::install(agent);
    cliff::install(agent);
}
