use {
    crate::imports::status_imports::*
};

unsafe extern "C" fn ryu_speciallw_step_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_speciallw_step_pre(fighter)
}

unsafe extern "C" fn ryu_speciallw_step_b_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_speciallw_step_pre(fighter)
}

unsafe extern "C" fn ryu_speciallw_step_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let keep_flag;
    let keep_int;
    let keep_float;
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT;
    }
    else {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep_flag,
        keep_int,
        keep_float,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ryu_speciallw_step_f_pre,
        ryu_speciallw_step_b_pre
    );
}