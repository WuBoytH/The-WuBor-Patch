use smash::{app::*, lib::lua_const::*};

#[skyline::hook(offset = 0x34f84d0)]
pub unsafe extern "C" fn richter_cross_on_hit(vtable: u64, weapon: &mut Weapon, hit_struct: u64) -> u64 {
    if (weapon.battle_object).kind == *WEAPON_KIND_RICHTER_CROSS as u32 {
        return 1;
    }
    call_original!(vtable, weapon, hit_struct)
}

pub fn install() {
    skyline::install_hooks!(
        richter_cross_on_hit
    );
}