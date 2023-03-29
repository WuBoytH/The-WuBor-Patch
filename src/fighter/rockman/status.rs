mod attack;
mod attack_s3;
mod attack_air;
mod special_n;
mod rockbuster;
pub mod helper;

pub fn install() {
    attack::install();
    attack_s3::install();
    attack_air::install();
    special_n::install();
    rockbuster::install();
}
