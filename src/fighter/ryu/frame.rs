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
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*,
        gameplay::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
fn ryu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Jump Cancel Heavy Up-Tilt

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLAG_JUMP_CANCEL) {
            jump_cancel_check_hit(fighter, false);
        }

        // Reset Vars

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_EX_FOCUS);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH);
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_DISABLE_EX_FOCUS_TIMER);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SECRET_SENSATION);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE);
            // OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
        }

        // EX Focus Attack Check
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_EX_FOCUS) {
            if (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n")
            && MotionModule::frame(fighter.module_accessor) > 13.0)
            || (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s_start") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s")
            && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)))
            || (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_command")
            && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS);
            }
            else if WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS)
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS);
            }
        }

        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS)
        && !StopModule::is_stop(fighter.module_accessor) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH);
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
            if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
                WorkModule::set_float(fighter.module_accessor, 1200.0, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_DISABLE_EX_FOCUS_TIMER);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_EX_FOCUS);
            }
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FOCUS);
        }

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_DISABLE_EX_FOCUS_TIMER) > 0.0 {
            count_down(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_DISABLE_EX_FOCUS_TIMER, 1.0);
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_DISABLE_EX_FOCUS_TIMER) <= 0.0 {
                let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                let focuseff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                EffectModule::set_rgb(fighter.module_accessor, focuseff, 0.0, 0.0, 0.0);
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_EX_FOCUS);
            }
        }

        // EX Flash

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH) {
            let mut flash_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE) {
                if flash_timer == 0 {
                    WorkModule::set_int(fighter.module_accessor, 8, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
                    flash_timer = 8;
                }
                if flash_timer <= 4 {
                    macros::COL_NORMAL(fighter);
                    WorkModule::dec_int(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
                }
                if flash_timer > 4 {
                    macros::FLASH(fighter, 0, 0.55, 1, 1.0);
                    WorkModule::dec_int(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
                }
            }
            else {
                if flash_timer == 0 {
                    WorkModule::set_int(fighter.module_accessor, 13, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
                }
                else if flash_timer == 1 {
                    macros::COL_NORMAL(fighter);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH);
                    WorkModule::dec_int(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
                }
                else {
                    macros::FLASH(fighter, 1, 1, 0.0, 0.75);
                    WorkModule::dec_int(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
                }
            }
        }

        // Secret Sensation Code

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY)
        || DamageModule::damage(fighter.module_accessor, 0) >= 200.0 {
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
            && MotionModule::frame(fighter.module_accessor) > WorkModule::get_float(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLOAT_HIT_FRAME) + 1.0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, 0x1daca540be as u64);
                sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }

            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SECRET_SENSATION) {
                StopModule::end_stop(fighter.module_accessor);
                JostleModule::set_status(fighter.module_accessor, false);
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
                DamageModule::set_damage_lock(fighter.module_accessor, true);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8 }, 0.0);
                if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_CAMERA) {
                    macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_exec"));
                    if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
                        macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 1.5, 0.0, 0.0);
                    }
                    let target_id = WorkModule::get_int64(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID) as u32;
                    if sv_battle_object::is_active(target_id) {
                        let target_boma = sv_battle_object::module_accessor(target_id);
                        SlowModule::set(
                            target_boma,
                            0,
                            100,
                            32,
                            false,
                            0x50000000
                        );
                    }
                    SlowModule::set_whole(fighter.module_accessor, 4, 0);
                    macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                    let ryu_x = PostureModule::pos_x(fighter.module_accessor);
                    let mut ryu_y = PostureModule::pos_y(fighter.module_accessor);
                    if ryu_x == WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_X) {
                        let opp_direc = 12.0 * PostureModule::lr(fighter.module_accessor);
                        WorkModule::set_float(fighter.module_accessor, opp_direc, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_OPPONENT_DIREC);
                    }
                    else if ryu_x < WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_X) {
                        WorkModule::set_float(fighter.module_accessor, 12.0, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_OPPONENT_DIREC);
                        if PostureModule::lr(fighter.module_accessor) == -1.0 {
                            PostureModule::reverse_lr(fighter.module_accessor);
                        }
                    }
                    else {
                        WorkModule::set_float(fighter.module_accessor, -12.0, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_OPPONENT_DIREC);
                        if PostureModule::lr(fighter.module_accessor) == 1.0 {
                            PostureModule::reverse_lr(fighter.module_accessor);
                        }
                    }
                    if sv_battle_object::is_active(target_id) && {
                        let target_boma = sv_battle_object::module_accessor(target_id);
                        (ryu_y - WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_Y)).abs() <= 12.0
                        && StatusModule::situation_kind(target_boma) == *SITUATION_KIND_GROUND }
                    {
                        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_VERT_EXTRA);
                    }
                    else {
                        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                        WorkModule::set_float(fighter.module_accessor, 12.0, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_VERT_EXTRA);
                        ryu_y += 2.0;
                        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                            x: 0.0,
                            y: 2.0
                        });
                    }
                    WorkModule::set_float(fighter.module_accessor, ryu_x, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_RYU_X);
                    WorkModule::set_float(fighter.module_accessor, ryu_y, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_RYU_Y);
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_CAMERA);
                }
                if WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER) >= 0.0 {
                    let target_id = WorkModule::get_int64(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID) as u32;
                    let ryu_x = WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_RYU_X);
                    let ryu_y = WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_RYU_Y);
                    let target_x = WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_X);
                    let target_y = WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_Y);
                    if sv_battle_object::is_active(target_id) {
                        let target_boma = sv_battle_object::module_accessor(target_id);
                        if (ryu_y - target_y).abs() <= 12.0
                        && StatusModule::situation_kind(target_boma) == *SITUATION_KIND_GROUND {
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                        }
                    }
                    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F {
                        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F, true);
                    }
                    if (ryu_y - target_y).abs() > 12.0 {
                        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                    }
                    let sec_sen_timer = WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER);
                    let opp_direc = WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_OPPONENT_DIREC);
                    let vert_extra = WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_VERT_EXTRA);
                    PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{
                        x: (((target_x + opp_direc) * sec_sen_timer) + ryu_x * (1.0 - sec_sen_timer)),
                        y: (((target_y + vert_extra) * sec_sen_timer) + ryu_y * (1.0 - sec_sen_timer))
                    });
                }
                add_f32(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER, 0.08);
                if WorkModule::get_float(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER) > 1.0 {
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SECRET_SENSATION);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_CAMERA);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                    macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_TURN_AUTO, true);
                    }
                    if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
                        macros::CAM_ZOOM_OUT(fighter);
                    }
                    macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
                    SlowModule::clear_whole(fighter.module_accessor);
                    JostleModule::set_status(fighter.module_accessor, true);
                    WorkModule::set_float(fighter.module_accessor, -0.6, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_SEC_SEN_TIMER);
                }
            }
            else if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r")
            || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") {
                if MotionModule::frame(fighter.module_accessor) == 4.0 {
                    macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_aura"));
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE);
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH);
                    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_RYU_INSTANCE_WORK_ID_INT_FLASH_TIMER);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
                    macros::BURN_COLOR(fighter, 0.0, 0.55, 1.0, 0.7);
                }
                if MotionModule::frame(fighter.module_accessor) <= 30.0
                && MotionModule::frame(fighter.module_accessor) >= 4.0 {
                    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
                    DamageModule::set_damage_lock(fighter.module_accessor, true);
                }
                else {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("ryu_savingattack_aura"), false, true);
                    macros::BURN_COLOR_NORMAL(fighter);
                    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
                    DamageModule::set_damage_lock(fighter.module_accessor, false);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH);
                    macros::COL_NORMAL(fighter);
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE);
                    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
                }
            }
            else if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SECRET_SENSATION)
            && WorkModule::is_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE) {
                DamageModule::set_damage_lock(fighter.module_accessor, false);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_EX_FLASH);
                macros::COL_NORMAL(fighter);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("ryu_savingattack_aura"), false, true);
                macros::BURN_COLOR_NORMAL(fighter);
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE);
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ryu_frame
    );
}