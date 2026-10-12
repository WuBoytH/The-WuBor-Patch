use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::captain::ON_ATTACK_ORIGINAL)]
pub fn captain_on_attack_original(vtable: u64, fighter: &mut Fighter, log: u64);

pub unsafe extern "C" fn captain_on_frame(_vtable: u64, fighter: &mut Fighter) {
    let object = (*fighter).battle_object;
    let module_accessor = object.module_accessor;
    captain_check_boost_power(module_accessor);
}

pub unsafe extern "C" fn captain_on_attack(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32) {
    let object = (*fighter).battle_object;
    let module_accessor = object.module_accessor;
    let kind = object.kind;
    let collision_log = log as *mut CollisionLogScuffed;
    if (*collision_log).collision_kind == *COLLISION_KIND_HIT as u8 {
        let opponent_object = MiscModule::get_battle_object_from_id((*collision_log).opponent_object_id);
        if (*opponent_object).battle_object_id >> 0x1c == 0
        && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
            captain_update_boost_power(module_accessor, damage);
        }
    }

    if kind == *FIGHTER_KIND_CAPTAIN as u32 {
        let status = StatusModule::status_kind(module_accessor);
        if [
            *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END,
        ].contains(&status) {
            if VarModule::is_flag(module_accessor, vars::captain::status::flag::USED_BOOST_POWER)
            && VarModule::is_flag(module_accessor, vars::captain::status::flag::ENABLE_BOOST_POWER_CRITICAL) {
                MiscModule::call_critical(module_accessor, log, 0xb, hash40("param_special_n"), 1, 0, 0, 0, 0);
                return;
            }
        }
    }
    captain_on_attack_original(vtable, fighter, log)
}

pub unsafe extern "C" fn captain_on_damage(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let object = (*fighter).battle_object;
    let module_accessor = object.module_accessor;
    let damage = *((*(log as *const u64).add(0x10 / 0x8)) as *const f32).add(0x4 / 0x4);
    captain_update_boost_power(module_accessor, -damage);
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_CAPTAIN,
        13,
        captain_on_frame as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_CAPTAIN,
        36,
        captain_on_attack as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_CAPTAIN,
        68,
        captain_on_damage as *const () as _
    );
}
