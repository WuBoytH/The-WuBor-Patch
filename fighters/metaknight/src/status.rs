use super::*;

mod attack_s3;

pub fn install(agent: &mut Agent) {
    attack_s3::install(agent);
}