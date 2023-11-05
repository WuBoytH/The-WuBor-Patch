use crate::imports::status_imports::*;

pub unsafe extern "C" fn luigi_remove_thunderhand_eff(fighter: &mut L2CFighterCommon) {
    if ![
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_thunder"), false, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("luigi_rocket_hold"), false, true);
    }
}