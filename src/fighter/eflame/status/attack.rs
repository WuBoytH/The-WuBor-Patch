use {
    smash::{
        lua2cpp::*,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*
};

#[status_script(agent = "eflame", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn eflame_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    eflame_sub_status_attackcommon(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

unsafe extern "C" fn eflame_sub_status_attackcommon(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
    }
    let combo = ComboModule::count(fighter.module_accessor) as i32;
    let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD)
    || attack_combo_max <= combo {
        ComboModule::reset(fighter.module_accessor);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100);
    WorkModule::set_int64(fighter.module_accessor, hash40("attack_11") as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
}

pub fn install() {
    install_status_scripts!(
        eflame_attack_main
    );
}