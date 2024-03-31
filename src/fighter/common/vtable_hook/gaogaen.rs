use crate::imports::*;

#[skyline::hook(offset = 0xab9970)]
pub unsafe extern "C" fn gaogaen_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    if [
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_TURN
    ].contains(&status)
    && (*collision_log).collision_kind == 1
    && VarModule::is_flag(module_accessor, gaogaen::status::flag::REVENGE_CRITICAL) {
        MiscModule::call_critical(module_accessor, log, 0x47, 0x18dfb2f4 | 0x1000000000, 1, 0, 0, 0);
        VarModule::off_flag(module_accessor, gaogaen::status::flag::REVENGE_CRITICAL);
        return;
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
    skyline::install_hooks!(
        gaogaen_on_attack
    );
    // Skips to the end of the Revenge check after changing statuses
    skyline::patching::Patch::in_text(0xab9ff0).data(0x140000D1u32);
}