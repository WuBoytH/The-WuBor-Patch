use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::weapon::edge_flare1::INIT)]
unsafe extern "C" fn flare1_init_original(vtable: u64, weapon: &mut smash::app::Weapon, init_args: u64);

unsafe extern "C" fn flare1_init(vtable: u64, weapon: &mut smash::app::Weapon, init_args: u64) {
    let module_accessor = weapon.battle_object.module_accessor;

    let mut power = *(init_args as *mut f32).add(0x88 / 0x4);
    let mut ratio = *(init_args as *mut f32).add(0x8c / 0x4);
    // let angle = *(something as *mut f32).add(0x90 / 0x4);
    // println!("Power: {}, Ratio: {}, Angle: {}", power, ratio, angle);

    if ratio > 1.0 {
        ratio = 0.0;
        power = 1.0;
        VarModule::on_flag(module_accessor, vars::edge_flare1::instance::flag::IS_GUARD_CANCEL);
    }

    *(init_args as *mut f32).add(0x88 / 0x4) = power;
    *(init_args as *mut f32).add(0x8c / 0x4) = ratio;

    flare1_init_original(vtable, weapon, init_args)
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_EDGE_FLARE1,
        0x7,
        flare1_init as *const () as _
    );
}
