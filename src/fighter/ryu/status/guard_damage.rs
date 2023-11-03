use crate::imports::status_imports::*;

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ryu_guard_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardDamage_common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_guard_damage_main_loop as *const () as _))
}

unsafe fn ryu_guard_damage_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IS_STOP].get_bool()
    && VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), true.into());
        return 1.into();
    }
    fighter.status_GuardDamage_Main()
}

pub fn install() {
    install_status_scripts!(
        ryu_guard_damage_main
    );
}