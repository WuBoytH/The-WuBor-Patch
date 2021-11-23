use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    crate::{
        common_funcs::*,
        vars::*
    }
};

pub unsafe extern "C" fn yu_specialns_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if AIR_ACTION[entry_id(fighter.module_accessor)] {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn yu_speciallw_pre(_fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

pub unsafe extern "C" fn yu_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND);
    if SRK[entry_id(fighter.module_accessor)] > 3 {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
            return true.into();
        }
    }
    if QCF[entry_id(fighter.module_accessor)] > 3 {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
            return true.into();
        }
    }
    if QCB[entry_id(fighter.module_accessor)] > 3 {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S) {
            ControlModule::reset_flick_sub_x(fighter.module_accessor);
            ControlModule::reset_main_stick(fighter.module_accessor);
            ControlModule::reset_turn_lr(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
            return true.into();
        }
    }
    0.into()
}
