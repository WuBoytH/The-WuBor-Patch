use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40, Vector3f, Vector2f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

unsafe fn ryu_reset_vars(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::DISABLE_EX_FOCUS);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::EX_FOCUS);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::EX_FLASH);
        VarModule::set_float(fighter.battle_object, ryu::instance::float::DISABLE_EX_FOCUS_TIMER, 0.0);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::SECRET_SENSATION);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_STATE);
    }
}

// move to status eventually
unsafe fn ryu_ex_focus(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, ryu::instance::flag::DISABLE_EX_FOCUS) {
        let mut can_exfadc = false;
        if ([*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND].contains(&fighter.global_table[STATUS_KIND].get_i32())
        && MotionModule::frame(fighter.module_accessor) > 16.0)
        || ([
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
            *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
            ].contains(&fighter.global_table[STATUS_KIND].get_i32())
        && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))) {
            can_exfadc = true;
        }
        if can_exfadc && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            VarModule::on_flag(fighter.battle_object, ryu::instance::flag::EX_FOCUS);
        }
        else if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::EX_FOCUS)
        && ![*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
            VarModule::off_flag(fighter.battle_object, ryu::instance::flag::EX_FOCUS);
        }
    }

    if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::EX_FOCUS)
    && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && !fighter.global_table[IS_STOP].get_bool() {
        if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4N4 != 0 {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), true.into());
        }
        else if fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6 != 0 {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F.into(), true.into());
        }
        else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        }
    }

    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B].contains(&fighter.global_table[STATUS_KIND].get_i32())
    && VarModule::is_flag(fighter.battle_object, ryu::instance::flag::EX_FOCUS) {
        VarModule::on_flag(fighter.battle_object, ryu::instance::flag::EX_FLASH);
        VarModule::set_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER, 0);
        VarModule::set_float(fighter.battle_object, ryu::instance::float::DISABLE_EX_FOCUS_TIMER, 1200.0);
        VarModule::on_flag(fighter.battle_object, ryu::instance::flag::DISABLE_EX_FOCUS);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::EX_FOCUS);
    }

    if VarModule::get_float(fighter.battle_object, ryu::instance::float::DISABLE_EX_FOCUS_TIMER) > 0.0 {
        VarModule::sub_float(fighter.battle_object, ryu::instance::float::DISABLE_EX_FOCUS_TIMER, 1.0);
        if VarModule::get_float(fighter.battle_object, ryu::instance::float::DISABLE_EX_FOCUS_TIMER) <= 0.0 {
            let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
            let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
            let focuseff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rgb(fighter.module_accessor, focuseff, 0.0, 0.0, 0.0);
            VarModule::off_flag(fighter.battle_object, ryu::instance::flag::DISABLE_EX_FOCUS);
        }
    }
}

unsafe fn ryu_ex_flash(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::EX_FLASH) {
        let mut flash_timer = VarModule::get_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER);
        if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_STATE) {
            if flash_timer == 0 {
                VarModule::set_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER, 8);
                flash_timer = 8;
            }
            if flash_timer <= 4 {
                macros::COL_NORMAL(fighter);
                VarModule::dec_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER);
            }
            if flash_timer > 4 {
                macros::FLASH(fighter, 0, 0.55, 1, 1.0);
                VarModule::dec_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER);
            }
        }
        else {
            if flash_timer == 0 {
                VarModule::set_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER, 13);
            }
            else if flash_timer == 1 {
                macros::COL_NORMAL(fighter);
                VarModule::off_flag(fighter.battle_object, ryu::instance::flag::EX_FLASH);
                VarModule::dec_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER);
            }
            else {
                macros::FLASH(fighter, 1, 1, 0.0, 0.75);
                VarModule::dec_int(fighter.battle_object, ryu::instance::int::FLASH_TIMER);
            }
        }
    }

}

unsafe fn ryu_secret_sensation_hit_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
    && [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&StatusModule::status_kind(fighter.module_accessor))
    && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
    && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && !fighter.global_table[IS_STOP].get_bool() {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new_raw(0x1daca540be));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
}

unsafe fn ryu_secret_sensation(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, ryu::instance::flag::SECRET_SENSATION) {
        StopModule::end_stop(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, false);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        DamageModule::set_damage_lock(fighter.module_accessor, true);
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true);
        HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8 }, 0.0);
        if !VarModule::is_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_CAMERA) {
            macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_exec"));
            if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
                macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 1.5, 0.0, 0.0);
            }
            let target_id = VarModule::get_int(fighter.battle_object, commons::instance::int::TARGET_ID) as u32;
            if sv_battle_object::is_active(target_id) {
                let target_boma = sv_battle_object::module_accessor(target_id);
                SlowModule::set(
                    target_boma,
                    0,
                    100,
                    32,
                    false,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
            }
            SlowModule::set_whole(fighter.module_accessor, 4, 0);
            macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
            let ryu_x = PostureModule::pos_x(fighter.module_accessor);
            let mut ryu_y = PostureModule::pos_y(fighter.module_accessor);
            if ryu_x == VarModule::get_float(fighter.battle_object, ryu::instance::float::TARGET_X) {
                let opp_direc = 12.0 * PostureModule::lr(fighter.module_accessor);
                VarModule::set_float(fighter.battle_object, ryu::instance::float::OPPONENT_DIREC, opp_direc);
            }
            else if ryu_x < VarModule::get_float(fighter.battle_object, ryu::instance::float::TARGET_X) {
                VarModule::set_float(fighter.battle_object, ryu::instance::float::OPPONENT_DIREC, 12.0);
                if PostureModule::lr(fighter.module_accessor) == -1.0 {
                    PostureModule::reverse_lr(fighter.module_accessor);
                }
            }
            else {
                VarModule::set_float(fighter.battle_object, ryu::instance::float::OPPONENT_DIREC, -12.0);
                if PostureModule::lr(fighter.module_accessor) == 1.0 {
                    PostureModule::reverse_lr(fighter.module_accessor);
                }
            }
            if sv_battle_object::is_active(target_id) && {
                let target_boma = sv_battle_object::module_accessor(target_id);
                (ryu_y - VarModule::get_float(fighter.battle_object, ryu::instance::float::TARGET_Y)).abs() <= 12.0
                && StatusModule::situation_kind(target_boma) == *SITUATION_KIND_GROUND }
            {
                VarModule::set_float(fighter.battle_object, ryu::instance::float::VERT_EXTRA, 0.0);
            }
            else {
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                VarModule::set_float(fighter.battle_object, ryu::instance::float::VERT_EXTRA, 12.0);
                ryu_y += 2.0;
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 0.0,
                    y: 2.0
                });
            }
            VarModule::set_float(fighter.battle_object, ryu::instance::float::RYU_X, ryu_x);
            VarModule::set_float(fighter.battle_object, ryu::instance::float::RYU_Y, ryu_y);
            VarModule::on_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_CAMERA);
        }
        if VarModule::get_float(fighter.battle_object, ryu::instance::float::SEC_SEN_TIMER) >= 0.0 {
            let target_id = VarModule::get_int(fighter.battle_object, commons::instance::int::TARGET_ID) as u32;
            let ryu_x = VarModule::get_float(fighter.battle_object, ryu::instance::float::RYU_X);
            let ryu_y = VarModule::get_float(fighter.battle_object, ryu::instance::float::RYU_Y);
            let target_x = VarModule::get_float(fighter.battle_object, ryu::instance::float::TARGET_X);
            let target_y = VarModule::get_float(fighter.battle_object, ryu::instance::float::TARGET_Y);
            if sv_battle_object::is_active(target_id) {
                let target_boma = sv_battle_object::module_accessor(target_id);
                if (ryu_y - target_y).abs() <= 12.0
                && StatusModule::situation_kind(target_boma) == *SITUATION_KIND_GROUND {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, false);
            }
            if (ryu_y - target_y).abs() > 12.0 {
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            }
            let sec_sen_timer = VarModule::get_float(fighter.battle_object, ryu::instance::float::SEC_SEN_TIMER);
            let opp_direc = VarModule::get_float(fighter.battle_object, ryu::instance::float::OPPONENT_DIREC);
            let vert_extra = VarModule::get_float(fighter.battle_object, ryu::instance::float::VERT_EXTRA);
            PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{
                x: (((target_x + opp_direc) * sec_sen_timer) + ryu_x * (1.0 - sec_sen_timer)),
                y: (((target_y + vert_extra) * sec_sen_timer) + ryu_y * (1.0 - sec_sen_timer))
            });
        }
        VarModule::add_float(fighter.battle_object, ryu::instance::float::SEC_SEN_TIMER, 0.08);
        if VarModule::get_float(fighter.battle_object, ryu::instance::float::SEC_SEN_TIMER) > 1.0 {
            VarModule::off_flag(fighter.battle_object, ryu::instance::flag::SECRET_SENSATION);
            VarModule::off_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_CAMERA);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_TURN_AUTO, false);
            }
            if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
                macros::CAM_ZOOM_OUT(fighter);
            }
            macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
            SlowModule::clear_whole(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, true);
            VarModule::set_float(fighter.battle_object, ryu::instance::float::SEC_SEN_TIMER, -0.6);
        }
    }
    else if !VarModule::is_flag(fighter.battle_object, ryu::instance::flag::SECRET_SENSATION)
    && VarModule::is_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_STATE)
    && ![
        hash40("appeal_hi_l"),
        hash40("appeal_hi_r")
    ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
        DamageModule::set_damage_lock(fighter.module_accessor, false);
        DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
        HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::EX_FLASH);
        macros::COL_NORMAL(fighter);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("ryu_savingattack_aura"), false, true);
        macros::BURN_COLOR_NORMAL(fighter);
        VarModule::off_flag(fighter.battle_object, ryu::instance::flag::SEC_SEN_STATE);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
fn ryu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        ryu_reset_vars(fighter);
        ryu_ex_focus(fighter);
        ryu_ex_flash(fighter);
        if DamageModule::damage(fighter.module_accessor, 0) >= 180.0 {
            ryu_secret_sensation_hit_cancel(fighter);
            ryu_secret_sensation(fighter);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ryu_frame
    );
}