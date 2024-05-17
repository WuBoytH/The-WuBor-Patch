use super::*;

mod attack_s3;

pub fn install(agent: &mut smashline::Agent) {
    attack_s3::install(agent);
}