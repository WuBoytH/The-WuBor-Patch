use crate::imports::status_imports::*;

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn richter_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3();
    fighter.sub_shift_status_main(L2CValue::Ptr(richter_attack_lw3_main_loop as *const () as _))
}

unsafe extern "C" fn richter_attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0 { // used to be *FIGHTER_PAD_FLAG_ATTACK_TRIGGER
        fighter.change_status(FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32.into(), true.into());
        return 1.into();
    }
    fighter.status_AttackLw3_Main()
}

pub fn install() {
    install_status_scripts!(
        richter_attack_lw3_main
    );
}