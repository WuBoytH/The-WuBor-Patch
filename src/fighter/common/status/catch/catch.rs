use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_Catch)]
unsafe fn status_catch(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_Catch();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Catch_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchDash)]
unsafe fn status_catchdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchDash();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchDash_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_CatchTurn)]
unsafe fn status_catchturn(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchTurn();
    GrabModule::set_rebound(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchTurn_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_CatchCont)]
unsafe fn catchcont(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        let special = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0;
        if attack || special {
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
unsafe fn fighterstatuscapture_set_invalid_capture(_fighter: &mut L2CFighterCommon) {
    // Haha there's nothing here now
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_catch,
            status_catchdash,
            status_catchturn,
            catchcont,
            fighterstatuscapture_set_invalid_capture
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}