use crate::imports::*;

unsafe extern "C" fn sonic_on_init(_vtable: u64, fighter: &mut Fighter) {
    VarModule::on_flag(fighter.battle_object.module_accessor, fighter::instance::flag::CAN_LOOK_UP);
}

#[skyline::hook(offset = 0x11d5a00)]
unsafe extern "C" fn sonic_on_hit(vtable: u64, fighter: &mut Fighter, log: u64) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    let collision_kind = (*collision_log).collision_kind;
    if [
        sonic::status::SPECIAL_AIR_S_HOLD,
        sonic::status::SPECIAL_AIR_S_END
    ].contains(&status) {
        if collision_kind == 1 {
            VarModule::on_flag(module_accessor, sonic::status::flag::SPECIAL_AIR_S_CHECK_HIT);
        }
        if [1, 2].contains(&collision_kind)
        && status == sonic::status::SPECIAL_AIR_S_HOLD {
            VarModule::on_flag(module_accessor, sonic::status::flag::SPECIAL_AIR_S_TO_END);
            KineticModule::unable_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    original!()(vtable, fighter, log);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5045570).data(sonic_on_init as u64);
    skyline::install_hooks!(
        sonic_on_hit
    );
}