mod landing;
mod attack_s3;
mod attack_hi3;

pub fn install(agent: &mut smashline::Agent) {
    landing::install(agent);
    attack_s3::install(agent);
    attack_hi3::install(agent);
}