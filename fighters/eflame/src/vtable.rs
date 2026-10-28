use super::*;

pub unsafe extern "C" fn eflame_check_can_transition_status(_vtable: u64, _fighter: &mut Fighter, _status: i32) -> bool {
    // Vanilla Logic (Unused)
    // let module_accessor = _fighter.battle_object.module_accessor;
    // let has_sword = WorkModule::get_flag(module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD);
    // 2 < _status - 0x8d || has_sword
    true
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_EFLAME,
        FighterVtableSlot::CheckCanTransitionStatus,
        eflame_check_can_transition_status as *const () as _
    );
}
