mod damage;
mod damage_air;
mod damage_fall;
mod damage_fly;
mod damage_fly_roll;
// mod damage_fly_reflect;

pub fn install() {
    damage::install();
    damage_air::install();
    damage_fall::install();
    damage_fly::install();
    damage_fly_roll::install();
    // damage_fly_reflect::install();
}