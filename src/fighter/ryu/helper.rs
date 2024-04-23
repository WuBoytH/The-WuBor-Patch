use {
    crate::imports::*,
    crate::fighter::common::status::attack::attack::only_jabs
};

#[inline(always)]
pub unsafe fn ryu_saving_aura_handler(agent: &mut L2CAgentBase, r: f32, g: f32, b: f32) {
    if !VarModule::is_flag(agent.module_accessor, ryu::status::flag::SET_DENJIN_AURA) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
        macros::LAST_EFFECT_SET_COLOR(agent, r, g, b);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, r, g, b);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, r, g, b);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, r, g, b);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, r, g, b);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, r, g, b);
        VarModule::on_flag(agent.module_accessor, ryu::status::flag::SET_DENJIN_AURA);
    }
}

pub unsafe fn ryu_denjin_remover(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE);
    let eff_handle = VarModule::get_int(fighter.module_accessor, ryu::instance::int::DENJIN_EFF_HANDLE) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        EffectModule::kill(fighter.module_accessor, eff_handle, true, true);
    }
    VarModule::set_int(fighter.module_accessor, ryu::instance::int::DENJIN_EFF_HANDLE, 0);
}

pub unsafe extern "C" fn ryu_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT) {
    //     WorkModule::set_int64(fighter.module_accessor, 0x10556e6036, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
    //     WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    // }
    // else {
        WorkModule::set_int64(fighter.module_accessor, 0xb4f4e6f8f, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK11, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    // }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk as *const () as _));
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk4(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk4 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_main_loop as *const () as _))
}

pub unsafe extern "C" fn ryu_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
            return 1.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
            return 1.into();
        }
    }
    if ComboModule::count(fighter.module_accessor) == 1
    && !CancelModule::is_enable_cancel(fighter.module_accessor) {
        let current_frame = fighter.global_table[STATUS_FRAME].get_f32();
        let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
        if current_frame < attack_start_cancel_frame
        && ryu_kara_cancel(fighter).get_bool() {
            return 1.into();
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        let normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        ].to_vec();
        if normal_cancel_common(fighter, normal_cancels).get_bool() {
            return 1.into();
        }
    }
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("attack_11_w"),
        hash40("attack_11_s"),
        hash40("attack_11_near_s")
    ].contains(&mot) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER)
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
            let cont = if stick_y >= attack_hi3_stick_y {
                false
            }
            else {
                let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                if attack_lw3_stick_y >= stick_y {
                    false
                }
                else {
                    let stick_x = fighter.global_table[STICK_X].get_f32();
                    let attack_s3_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_x"));
                    stick_x < attack_s3_stick_x
                }
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL) {
            let button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
            let attack_11_s_button_on_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_11_s_button_on_frame"));
            if attack_11_s_button_on_frame <= button_on_frame as f32 {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME)
    && !StopModule::is_stop(fighter.module_accessor)
    && fighter.sub_check_button_jump().get_bool() {
        // let mot = MotionModule::motion_kind(fighter.module_accessor);
        // let func = ryu_get_attack_cancel_function(fighter, mot.into()).get_u64();
        // MotionAnimcmdModule::call_script_single(
        //     fighter.module_accessor,
        //     *FIGHTER_ANIMCMD_EXPRESSION,
        //     Hash40::new_raw(func),
        //     -1
        // );
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
        fighter.change_status_jump_mini_attack(true.into());
        return 1.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
        if !MotionModule::is_end(fighter.module_accessor) {
            ryu_idkwhatthisis2(fighter);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
    }
    0.into()
}

pub unsafe extern "C" fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo_count = ComboModule::count(fighter.module_accessor) as i32;
        let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
        if combo_count < attack_combo_max {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                return 0.into();
            }
            ryu_attack_main_uniq_chk2(fighter, hash40("attack_11_s").into(), hash40("attack_11_w").into());
        }
    }
    else {
        ryu_attack_main_uniq_chk2(fighter, hash40("attack_11_s").into(), hash40("attack_11_w").into());
    }
    0.into()
}

pub unsafe extern "C" fn ryu_attack_main_uniq_chk2(fighter: &mut L2CFighterCommon, mot1: L2CValue, mot2: L2CValue) {
    ryu_attack_reset(fighter);
    fighter.attack_mtrans_pre_process();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if ComboModule::count(fighter.module_accessor) == 1 {
            // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT) {
            //     WorkModule::set_int64(fighter.module_accessor, 0x10556e6036, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
            //     WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
            // }
            // else {
                WorkModule::set_int64(fighter.module_accessor, 0xb4f4e6f8f, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK11, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
            // }
        }
    }
    fighter.attack_mtrans_post_process();
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x265a5c1b6b), mot1.get_int(), mot2.get_int());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    if 1 < ComboModule::count(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
    }
}

pub unsafe extern "C" fn ryu_attack_reset(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
}

pub unsafe extern "C" fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            if !WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            }
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER) {
                return 0.into();
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && !VarModule::is_flag(fighter.module_accessor, attack::flag::INVALID_HOLD_INPUT) {
                let stick_y = fighter.global_table[STICK_Y].get_f32();
                let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
                let cont = if stick_y >= attack_hi3_stick_y {
                    false
                }
                else {
                    let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                    attack_lw3_stick_y < stick_y
                };
                if cont {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
                    return 0.into();
                }
            }
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
        }
    }
    else {
        fighter.attack_uniq_chk();
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            if only_jabs(fighter) {
                VarModule::off_flag(fighter.module_accessor, attack::flag::INVALID_HOLD_INPUT);
            }
            else {
                VarModule::on_flag(fighter.module_accessor, attack::flag::INVALID_HOLD_INPUT);
            }
        }
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
            }
        }
        else if !VarModule::is_flag(fighter.module_accessor, attack::flag::INVALID_HOLD_INPUT) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
                else {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                let count = ComboModule::count(fighter.module_accessor) as i32;
                let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
                if count & attack_combo_max == 0 {
                    return 0.into();
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                    return 0.into();
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
    }
    0.into()
}

pub unsafe extern "C" fn ryu_attack_s3_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_reset(fighter);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_TURN_RUN {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
    }
    ControlModule::reset_trigger(fighter.module_accessor);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_chara_dir,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        lr
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_s3_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_attack_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
        let frame = fighter.global_table[STATUS_FRAME].get_f32();
        let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
        if frame <= attack_start_cancel_frame {
            if ryu_kara_cancel(fighter).get_bool() {
                return 1.into();
            }
        }
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }

    if StatusModule::is_changing(fighter.module_accessor)
    || {
        let count = ComboModule::count(fighter.module_accessor) as i32;
        let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
        count < s3_combo_max &&
        WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) &&
        WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
    } {
        fighter.attack_s3_mtrans();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    // Normally has code to allow attack cancels, but attack canceling no longer exists...

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }

    0.into()
}

pub unsafe extern "C" fn ryu_attack_hi3_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let allow_jump_mini_attack = status == prev_status && FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true);
    ryu_attack_reset(fighter);
    fighter.clear_lua_stack();
    let mot = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let info = ryu_get_mini_jump_attack_data_log_info(fighter, mot.into());
    WorkModule::set_int64(fighter.module_accessor, info.get_u64() as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x265a5c1b6b), Hash40::new("attack_hi3_s"), Hash40::new("attack_hi3_w"));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    let fb = ControlModule::get_attack_hi3_fb_kind(fighter.module_accessor);
    if fb == *FIGHTER_COMMAND_ATTACK3_KIND_B {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_chara_dir,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            lr
        );
    }
    if allow_jump_mini_attack {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
            WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_hi3_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_attack_hi3_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_hi3_main_loop as *const() as _))
}

unsafe extern "C" fn ryu_attack_hi3_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let stop = param_1.get_bool();
    fighter.sub_attack3_uniq_check(stop.into());
    if stop
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
            if stick_y >= attack_hi3_stick_y {
                WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
            }
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
        }
    }
    0.into()
}

unsafe extern "C" fn ryu_attack_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
        let frame = fighter.global_table[STATUS_FRAME].get_f32();
        let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
        if frame <= attack_start_cancel_frame {
            if ryu_kara_cancel(fighter).get_bool() {
                return 1.into();
            }
        }
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER)
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
            if stick_y >= attack_hi3_stick_y {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), false.into());
                return 1.into();
            }
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL) {
        let button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
        let attack_hi3_s_button_on_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_hi3_s_button_on_frame"));
        if attack_hi3_s_button_on_frame <= button_on_frame as f32 {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), false.into());
            return 1.into();
        }
    }

    // Normally has code to allow f tilt to transition into the next hit, I did not want to rewrite it...

    // Normally has code to allow attack cancels, but attack canceling no longer exists...

    if !MotionModule::is_end(fighter.module_accessor) {
        ryu_check_attack_button_trigger(fighter);
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    
    0.into()
}

unsafe extern "C" fn ryu_check_attack_button_trigger(fighter: &mut L2CFighterCommon) {
    if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
    }
}

pub unsafe extern "C" fn ryu_attack_lw3_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_reset(fighter);
    fighter.status_AttackLw3_common_param(
        true.into(),
        L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _),
        0.into()
    );
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x265a5c1b6b), Hash40::new("attack_lw3_s"), Hash40::new("attack_lw3_w"));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_lw3_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_attack_lw3_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_lw3_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_attack_lw3_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let stop = param_1.get_bool();
    fighter.sub_attack3_uniq_check(stop.into());
    if stop
    && !SlowModule::is_skip(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                let stick_y = fighter.global_table[STICK_Y].get_f32();
                let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                if stick_y <= attack_lw3_stick_y {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
                }
            }
            else {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn ryu_attack_lw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
        let frame = fighter.global_table[STATUS_FRAME].get_f32();
        let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
        if frame <= attack_start_cancel_frame {
            if ryu_kara_cancel(fighter).get_bool() {
                return 1.into();
            }
        }
    }
    else {
        if fighter.sub_wait_ground_check_common(true.into()).get_bool() {
            return 1.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER)
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
            if stick_y <= attack_lw3_stick_y {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), false.into());
                return 1.into();
            }
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL) {
        let button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
        let attack_lw3_s_button_on_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_lw3_s_button_on_frame"));
        if attack_lw3_s_button_on_frame <= button_on_frame as f32 {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), false.into());
            return 1.into();
        }
    }

    // Normally has code to allow f tilt to transition into the next hit, I did not want to rewrite it...

    // Normally has code to allow attack cancels, but attack canceling no longer exists...

    if !MotionModule::is_end(fighter.module_accessor) {
        ryu_check_attack_button_trigger(fighter);
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    
    0.into()
}

unsafe extern "C" fn ryu_get_mini_jump_attack_data_log_info(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    let info = fighter.get_mini_jump_attack_data_log_info(param_1.clone());
    if info.get_bool() {
        info
    }
    else {
        let log = &*(((fighter as *const L2CFighterCommon as u64) + 0x228) as *const L2CValue);
        (*log)["log_infos"][param_1.clone()].clone()
    }
}

pub unsafe extern "C" fn ryu_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            mot = Hash40::new("special_air_hi_command");
        }
        else {
            mot = Hash40::new("special_air_hi");
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            mot = Hash40::new("special_hi_command");
        }
        else {
            mot = Hash40::new("special_hi");
        }
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    ItemModule::set_change_status_event(fighter.module_accessor, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_specialhi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_specialhi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_specialhi_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if !param_2.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
            if lr_stick_x < stick_x.abs() {
                PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ryu_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            ryu_specialhi_mot_helper(fighter);
        }
    }
    else {
        ryu_specialhi_mot_helper(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn ryu_specialhi_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if StatusModule::is_changing(fighter.module_accessor) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            let start_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_accel_y"));
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -start_accel_y
            );
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if StatusModule::is_changing(fighter.module_accessor) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
}

pub unsafe extern "C" fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let final_cancel = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    let ret = if situation.get_i32() != *SITUATION_KIND_GROUND {
        fighter.sub_transition_group_check_air_special().get_bool()
    }
    else {
        fighter.sub_transition_group_check_ground_special().get_bool()
    };
    if !final_cancel {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    }
    ret.into()
}

pub unsafe extern "C" fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let special_n = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    let special_s = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    let special_hi = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    let special_lw = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    let special_n_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    let special_n2_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let attack_command1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    let ret = if situation.get_i32() != *SITUATION_KIND_GROUND {
        fighter.sub_transition_group_check_air_special().get_bool()
    }
    else {
        fighter.sub_transition_group_check_ground_special().get_bool()
    };
    if !special_n {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    if !special_s {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    }
    if !special_hi {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
    if !special_lw {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    if !special_n_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    }
    if !special_n2_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    }
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !attack_command1 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    }
    ret.into()
}

pub unsafe extern "C" fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_n_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    let special_n2_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let attack_command1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    let ret = fighter.sub_transition_group_check_special_command().get_bool();
    if !special_n_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    }
    if !special_n2_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    }
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !attack_command1 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    }
    ret.into()
}

// pub unsafe extern "C" fn ryu_get_attack_cancel_function(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
//     let func = fighter.get_mini_jump_attack_data_cancel_function(param_1.clone());
//     if func.get_bool() == false {
//         println!("hi");
//         let val = &*(((fighter as *const L2CFighterCommon as u64) + 0x1) as *const L2CValue);
//         println!("val: {:?}", val);
//         let table = val[0x10f40d7b92 as u64].clone();
//         println!("table: {:?}", table);
//         let value = table[param_1.clone()].clone();
//         println!("value: {:?}", value);
//         value
//     }
//     else {
//         println!("func: {:?}", func);
//         func
//     }
// }

pub unsafe extern "C" fn ryu_idkwhatthisis2(fighter: &mut L2CFighterCommon) {
    if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && only_jabs(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
        }
    }
}
