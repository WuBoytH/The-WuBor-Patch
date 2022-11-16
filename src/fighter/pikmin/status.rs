mod landing;
mod attack_s3;
mod attack_hi3;

pub fn install() {
    landing::install();
    attack_s3::install();
    attack_hi3::install();
}