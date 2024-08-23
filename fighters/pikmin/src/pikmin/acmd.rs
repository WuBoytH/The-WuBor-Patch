use super::*;

mod catch;

pub fn install(agent: &mut Agent) {
    catch::install(agent);
}