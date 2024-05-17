use super::*;

extern "C" {
    #[link_name = "only_jabs"]
    pub fn only_jabs(fighter: &mut L2CFighterCommon) -> bool;
}

pub unsafe extern "C" fn marth_stance_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::marth::instance::flag::IS_STANCE) {
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
    if curr_status < vars::marth::status::STANCE_ATTACK_F3
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0
    && !is_back {
        fighter.change_status(vars::marth::status::STANCE_ATTACK_F3.into(), true.into());
        return true.into();
    }
    if curr_status < vars::marth::status::STANCE_ATTACK_B3
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0
    && is_back {
        fighter.change_status(vars::marth::status::STANCE_ATTACK_B3.into(), true.into());
        return true.into();
    }
    if curr_status < vars::marth::status::STANCE_ATTACK_HI3
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
        fighter.change_status(vars::marth::status::STANCE_ATTACK_HI3.into(), true.into());
        return true.into();
    }
    if curr_status < vars::marth::status::STANCE_ATTACK_LW3
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
        fighter.change_status(vars::marth::status::STANCE_ATTACK_LW3.into(), true.into());
        return true.into();
    }
    if curr_status < vars::marth::status::STANCE_ATTACK
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    && only_jabs(fighter) {
        fighter.change_status(vars::marth::status::STANCE_ATTACK.into(), true.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_special_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        return true.into();
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
        fighter.change_status(vars::marth::status::STANCE_SPECIAL_S.into(), true.into());
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
    let is_jab = curr_status == vars::marth::status::STANCE_ATTACK && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD);
    let cancel = CancelModule::is_enable_cancel(fighter.module_accessor) || is_jab;
    let cancel = cancel && !fighter.global_table[IS_STOP].get_bool();
    if cancel || !require_cancel {
        if VarModule::is_flag(fighter.module_accessor, vars::marth::instance::flag::IS_STANCE) {
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
                fighter.change_status(vars::marth::status::STANCE_DASH_B.into(), true.into());
                return true.into();
            }
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
            || dash_f {
                fighter.change_status(vars::marth::status::STANCE_DASH_F.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_mot_end_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if !VarModule::is_flag(fighter.module_accessor, vars::marth::instance::flag::IS_STANCE) {
            let clear_buffer = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0;
            fighter.change_status(vars::marth::status::STANCE_EXIT.into(), clear_buffer.into());
            return true.into();
        }
        else {
            fighter.change_status(vars::marth::status::STANCE_WAIT.into(), false.into());
        }
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status < vars::marth::status::STANCE_ENTER
    && status != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && status != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT
    && status != *FIGHTER_STATUS_KIND_SPECIAL_HI {
        VarModule::off_flag(fighter.module_accessor, vars::marth::instance::flag::IS_STANCE);
    }
    VarModule::off_flag(fighter.module_accessor, vars::marth::instance::flag::PARRY_XLU);
    0.into()
}
