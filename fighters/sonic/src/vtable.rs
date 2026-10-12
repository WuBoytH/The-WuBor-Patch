use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::sonic::ON_HIT)]
unsafe extern "C" fn sonic_on_hit_inner(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32);

unsafe extern "C" fn sonic_on_init(_vtable: u64, fighter: &mut Fighter) {
    VarModule::on_flag(fighter.battle_object.module_accessor, vars::fighter::instance::flag::CAN_LOOK_UP);
}

unsafe extern "C" fn sonic_on_hit(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    let collision_kind = (*collision_log).collision_kind;
    if [
        vars::sonic::status::SPECIAL_AIR_S_HOLD,
        vars::sonic::status::SPECIAL_AIR_S_HIT,
        vars::sonic::status::SPECIAL_AIR_S_LAUNCH,
    ].contains(&status) {
        if collision_kind == 1 {
            VarModule::on_flag(module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_HIT);
            if status == vars::sonic::status::SPECIAL_AIR_S_HOLD {
                VarModule::on_flag(module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_TO_HIT);
            }
        }
    }
    sonic_on_hit_inner(vtable, fighter, log, damage);
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_SONIC,
        4,
        sonic_on_init as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_SONIC,
        36,
        sonic_on_hit as *const () as _
    );
}
