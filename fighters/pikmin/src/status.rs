use super::*;

mod landing;
mod attack_s3;
mod attack_hi3;

mod special_hi;
mod special_hi_wait;

pub fn install(agent: &mut smashline::Agent) {
    landing::install(agent);
    attack_s3::install(agent);
    attack_hi3::install(agent);

    special_hi::install(agent);
    special_hi_wait::install(agent);
}