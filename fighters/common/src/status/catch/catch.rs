use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Catch_common)]
unsafe extern "C" fn status_pre_catch_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, // was MOTION
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64,
        param_1.get_u32(),
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_CatchDash_common)]
unsafe extern "C" fn status_pre_catchdash_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP, // was MOTION
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64,
        param_1.get_u32(),
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Catch)]
unsafe extern "C" fn status_catch(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_Catch();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Catch_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchDash)]
unsafe extern "C" fn status_catchdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        mul_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.65,
        0.0
    );
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchDash();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchDash_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchTurn)]
unsafe extern "C" fn status_catchturn(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchTurn();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchTurn_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_CatchDashPull)]
unsafe extern "C" fn status_pre_catchdashpull(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION, // was CATCH_DASH
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        true,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_CatchCont)]
unsafe extern "C" fn catchcont(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let mut throw_f = false;
        let mut throw_b = false;
        let mut throw_hi = false;
        let mut throw_lw = false;
        let throw_stick = fighter.IsThrowStick();
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
            if throw_stick[0x176d32be0u64].get_bool() {
                throw_f = true;
            }
            if throw_stick[0x171beeff9u64].get_bool() {
                throw_b = true;
            }
            if throw_stick[0x2d8932aacu64].get_bool() {
                throw_hi = true;
            }
            if throw_stick[0x246f0d2cbu64].get_bool() {
                throw_lw = true;
            }
        }
        let cat2 = fighter.global_table[CMD_CAT2].get_i32();
        let strans = if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F)
        && cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F != 0
        || throw_f {
            [fighter.global_table[THROW_F_STATUS_KIND].get_i32(),
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F]
        }
        else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B)
        && cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B != 0
        || throw_b {
            [fighter.global_table[THROW_B_STATUS_KIND].get_i32(),
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B]
        }
        else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI)
        && cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0
        || throw_hi {
            [fighter.global_table[THROW_HI_STATUS_KIND].get_i32(),
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI]
        }
        else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW)
        && cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0
        || throw_lw {
            [fighter.global_table[THROW_LW_STATUS_KIND].get_i32(),
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW]
        }
        else {
            [-1, -1]
        };
        if strans[0] != -1 {
            WorkModule::set_int(fighter.module_accessor, strans[1], *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(strans[0].into(), true.into());
            return true.into();
        }
        let attack = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0;
        let catch = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0;
        let special = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0;
        if attack || catch || special {
            let catch_attack_distance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("catch_attack_distance"));
            let scale = PostureModule::scale(fighter.module_accessor);
            let capture_pos_x_diff = CatchModule::capture_pos_x_diff(fighter.module_accessor);
            if catch_attack_distance * scale <= 0.0
            || capture_pos_x_diff <= catch_attack_distance * scale {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), true.into());
                return true.into();
            }
        }

        // [NEW] Allow Taunt inputs to trigger a manual Grab Release.
        // if fighter.global_table[CMD_CAT2].get_i32() & (
        //     *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW |
        //     *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R
        // ) != 0 {
        //     if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_WAIT {
        //         CatchModule::catch_cut(fighter.module_accessor, false, false);
        //         fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), true.into());
        //         return true.into();
        //     }
        // }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusCapture_set_invalid_capture)]
unsafe extern "C" fn fighterstatuscapture_set_invalid_capture(_fighter: &mut L2CFighterCommon) {
    // Haha there's nothing here now
}

#[skyline::hook(replace = L2CFighterCommon_FighterStatusCapture_set_invalid_capture_SwingGaogaen)]
unsafe extern "C" fn fighterstatuscapture_set_invalid_capture_swinggaogaen(_fighter: &mut L2CFighterCommon) {
    // Haha there's nothing here now
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_catch_common,
            status_pre_catchdash_common,
            status_catch,
            status_catchdash,
            status_catchturn,
            status_pre_catchdashpull,
            catchcont,
            fighterstatuscapture_set_invalid_capture,
            fighterstatuscapture_set_invalid_capture_swinggaogaen
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}