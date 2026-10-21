use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::gaogaen::ON_ATTACK)]
unsafe extern "C" fn gaogaen_on_attack_inner(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32);

pub unsafe extern "C" fn gaogaen_on_attack(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    if [
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_TURN
    ].contains(&status)
    && (*collision_log).collision_kind == 1
    && VarModule::is_flag(module_accessor, vars::gaogaen::status::flag::REVENGE_CRITICAL) {
        MiscModule::call_critical(module_accessor, log, 0x47, 0x18dfb2f4 | 0x1000000000, 1, 0, 0, 0, 0);
        VarModule::off_flag(module_accessor, vars::gaogaen::status::flag::REVENGE_CRITICAL);
        return;
    }
    gaogaen_on_attack_inner(vtable, fighter, log, damage)
}

pub fn install() {
    unsafe {
        // Skips to the end of the Revenge check after changing statuses
        let _ = skyline::patching::Patch::in_text(offsets::fighter::gaogaen::REVENGE_CHECK_SKIP_PATCH).data(0x140000D1u32);
    }

    replace_fighter_vtable(
        *FIGHTER_KIND_GAOGAEN,
        FighterVtableSlot::OnAttack,
        gaogaen_on_attack as *const () as _
    );
}
