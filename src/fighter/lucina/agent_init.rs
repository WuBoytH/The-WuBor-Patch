use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    super::helper::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

pub unsafe extern "C" fn yu_specialns_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn yu_speciallw_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn yu_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret = false;
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND);
    if !ret
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP) > 6
    && spent_meter_super(fighter.module_accessor)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4.into(), true.into());
        ret = true;
    }
    if !ret
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP) > 3
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        ret = true;
    }
    if !ret
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP) > 3
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N)
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
        ret = true;
    }
    if !ret
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP) > 3
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S)
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S) {
        ControlModule::reset_flick_sub_x(fighter.module_accessor);
        ControlModule::reset_main_stick(fighter.module_accessor);
        ControlModule::reset_turn_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
        ret = true;
    }
    if ret {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND);
    }
    ret.into()
}
