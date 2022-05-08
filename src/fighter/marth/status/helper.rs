use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::table_const::*,
    custom_status::*,
    crate::fighter::common::common_status::attack::only_jabs,
    super::super::vars::*
};

pub unsafe extern "C" fn marth_stance_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if marth_stance_special_cancel_helper(fighter).get_bool() {
                return true.into();
            }
        }
        else {
            if marth_stance_special_cancel_helper(fighter).get_bool()
            || marth_stance_ground_cancel_helper(fighter).get_bool () {
                return true.into();
            }
        }
    }
    else {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW);
        if fighter.sub_transition_group_check_ground_catch().get_bool()
        || fighter.sub_transition_group_check_ground_special().get_bool()
        || fighter.sub_transition_group_check_ground_attack().get_bool()
        || fighter.sub_transition_group_check_air_special().get_bool() 
        || fighter.sub_transition_group_check_air_attack().get_bool() {
            return true.into();
        }
        let cat2 = fighter.global_table[CMD_CAT2].get_i32();
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            if fighter.pop_lua_stack(1).get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                return true.into();
            }
        }
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            if fighter.pop_lua_stack(1).get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                return true.into();
            }
        }
        if cat2 & (*FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            if fighter.pop_lua_stack(1).get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                return true.into();
            }
        }
    }
    false.into()
}

pub unsafe extern "C" fn marth_ground_air_cancel_helper(fighter: &mut L2CFighterCommon, require_cancel: bool) -> L2CValue {
    if !require_cancel || CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if marth_stance_special_cancel_helper(fighter).get_bool()
            || marth_stance_ground_cancel_helper(fighter).get_bool()
            || marth_stance_dash_cancel_helper(fighter, require_cancel).get_bool() {
                return true.into();
            }
        }
        else {
            if marth_stance_special_cancel_helper(fighter).get_bool() {
                return true.into();
            }
        }
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_ground_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let curr_status = fighter.global_table[STATUS_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let turn = FighterControlModuleImpl::get_attack_s3_turn(fighter.module_accessor) as i32;
    let is_back = (lr == 1.0 && turn == *FIGHTER_COMMAND_TURN_LR_LEFT) || (lr == -1.0 && turn == *FIGHTER_COMMAND_TURN_LR_RIGHT);
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_B3);
    if curr_status < status
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0
    && is_back {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_F3);
    if curr_status < status
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0
    && !is_back {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_HI3);
    if curr_status < status
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3);
    if curr_status < status
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK);
    if curr_status < status
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    && only_jabs(fighter) {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_special_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    // let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3);
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        return true.into();
    }
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S);
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_dash_cancel_helper(fighter: &mut L2CFighterCommon, require_cancel: bool) -> L2CValue {
    let curr_status = fighter.global_table[STATUS_KIND].get_i32();
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK);
    let is_jab = curr_status == status && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD);
    let cancel = CancelModule::is_enable_cancel(fighter.module_accessor) || is_jab;
    let cancel = cancel && !fighter.global_table[IN_HITLAG].get_bool();
    if cancel || !require_cancel {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            let cat2 = fighter.global_table[CMD_CAT2].get_i32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let dash_f = if lr < 0.0 {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
            }
            else {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0
            };
            let dash_b = if lr < 0.0 {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0
            }
            else {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
            };
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0
            || dash_b {
                let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B);
                fighter.change_status(status.into(), true.into());
                return true.into();
            }
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
            || dash_f {
                let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F);
                fighter.change_status(status.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_mot_end_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
            let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT);
            let clear_buffer = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0;
            fighter.change_status(status.into(), clear_buffer.into());
            return true.into();
        }
        else {
            let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT);
            fighter.change_status(status.into(), false.into());
        }
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status < CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER)
    && status != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && status != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    }
    0.into()
}
