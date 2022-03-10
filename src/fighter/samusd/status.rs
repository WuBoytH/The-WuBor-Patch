use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CWeaponCommon, *},
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    super::vl,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samusd_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Wait()
}

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samusd_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_SAMUSD_STATUS_ATTACK_AIR_FLAG_START_FLOAT);
    fighter.sub_attack_air_common(true.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        samusd_attackair_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(samusd_attackair_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

unsafe extern "C" fn samusd_attackair_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n")
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_SAMUSD_STATUS_ATTACK_AIR_FLAG_START_FLOAT) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_N_FLOAT);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        fighter.clear_lua_stack();
        if sum_speed_y <= 0.0 {
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y * vl::param_private::attack_air_n_gravity_mul);
        }
        else {
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        }
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    }
    0.into()
}

#[status_script(agent = "samusd", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samusd_specialn_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_n_h"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_n_h"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(samusd_specialn_hold_main_loop as *const () as _))
}

unsafe extern "C" fn samusd_specialn_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_sit = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let curr_sit = fighter.global_table[SITUATION_KIND].get_i32();
    if prev_sit != curr_sit {
        let mot;
        if curr_sit == *SITUATION_KIND_GROUND {
            mot = Hash40::new("special_n_h");
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        else {
            mot = Hash40::new("special_air_n_h");
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    let fire_trigger = fighter.global_table[PAD_FLAG].get_i32() & (
        *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER | *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) != 0;
    if !fire_trigger {
        let mut cont = false;
        let mut int_to_set = 0;
        if curr_sit == *SITUATION_KIND_GROUND {
            if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_F;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_B;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
            && !cont {
                if fighter.sub_check_button_jump().get_bool() {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if ControlModule::is_enable_flick_jump(fighter.module_accessor)
            && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
            && !cont {
                if fighter.sub_check_button_frick().get_bool() {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.sub_check_command_guard().get_bool()
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            if cont {
                WorkModule::set_int(fighter.module_accessor, int_to_set, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
                return 1.into();
            }
        }
        let mut next_status = 0;
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0
        && next_status == 0 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                next_status = *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C;
            }
        }
        else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
        && next_status == 0 {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT)
            < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
                        int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL;
                    }
                    else {
                        int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                    }
                    next_status = *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL;
                }
            }
        }
        else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
        && next_status == 0 {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT)
            < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                next_status = *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL;
            }
        }
        if next_status != 0 {
            WorkModule::set_int(fighter.module_accessor, int_to_set, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(next_status.into(), true.into());
            return 1.into();
        }
        if fighter.global_table[IN_HITLAG].get_bool() {
            let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let charge_max = samusd_get_max_charge_frame(fighter).get_f32();
            let ratio = charge as f32 / charge_max;
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x26b38955ef), ratio);
            return 0.into();
        }
        else {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let charge_max = samusd_get_max_charge_frame(fighter).get_f32();
            if charge >= charge_max as i32 {
                WorkModule::set_int(fighter.module_accessor, charge_max as i32, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
                let mot;
                if curr_sit != *SITUATION_KIND_GROUND {
                    mot = Hash40::new("special_air_n_f_max");
                }
                else {
                    mot = Hash40::new("special_n_f_max");
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
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), false.into());
                return 1.into();
            }
        }
        return 0.into();
    }
    else {
        let mot;
        if curr_sit != *SITUATION_KIND_GROUND {
            mot = Hash40::new("special_air_n_f");
        }
        else {
            mot = Hash40::new("special_n_f");
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
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), false.into());
    }
    1.into()
}

#[status_script(agent = "samusd", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn samusd_specialn_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F
    || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E {
        return 0.into();
    }
    if ![
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B
    ].contains(&status_kind) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        let charge_max_efh = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
        effect!(fighter, MA_MSC_EFFECT_REMOVE, charge_max_efh);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

unsafe extern "C" fn samusd_get_max_charge_frame(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")).into()
}

#[status_script(agent = "samusd", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samusd_speciallw_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    samusd_speciallw_helper(fighter);
    samusd_speciallw_air_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(samusd_speciallw_air_main_loop as *const () as _))
}

unsafe extern "C" fn samusd_speciallw_helper(fighter: &mut L2CFighterCommon) {
    let body = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_LW_BODY);
    if body != 1 {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x298600da7a));
    }
    else {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2993ae42dd));
    }
}

unsafe extern "C" fn samusd_speciallw_air_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot = Hash40::new("special_air_lw");
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MOT_RESTART) {
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
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MOT_RESTART);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

unsafe extern "C" fn samusd_speciallw_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if samusd_speciallw_air_is_end_helper(fighter).get_i32() == 1 {
        return 1.into();
    }
    if fighter.global_table[MOTION_FRAME].get_f32() >= 1.0 {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if AttackModule::is_attack(fighter.module_accessor, 0, false) {
                WorkModule::set_float(fighter.module_accessor, vl::param_special_lw::landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn samusd_speciallw_air_is_end_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if sit == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

#[status_script(agent = "samusd_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn samusd_cshot_shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    if utility::get_kind(&mut *oboma)  == *FIGHTER_KIND_SAMUSD {
        // WorkModule::on_flag(oboma, FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_CSHOT_ACTIVE);
        WorkModule::set_int(oboma, weapon.global_table[OBJECT_ID].get_i32(), FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_CSHOT_ID);
    }
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED)
    && !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samusd_cshot_bullet"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            1.0,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    }
    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let angle = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("angle")) as f32;
    let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_speed"));
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_speed"));
    let speed = (max_speed - min_speed) * charge + min_speed;
    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    sv_kinetic_energy::set_stable_speed(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -0.15 * lr, 0.0);
    sv_kinetic_energy::set_accel(weapon.lua_state_agent);
    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let min_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
        let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_scale"));
        let scale = (max_scale - min_scale) * charge + min_scale;
        if scale < 1.0
        && scale >= 0.3 {
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samusd_cshot_bullet_sub_a"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                scale,
                false,
                0,
                0,
                0
            );
        }
        else {
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samusd_cshot_bullet_sub_b"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                scale,
                false,
                0,
                0,
                0
            );
        }
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samusd_cshot_bullet_sub"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            scale,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);
    }
    0.into()
}

#[status_script(agent = "samusd_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn samusd_cshot_shoot_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_TYPE_NORMAL);
    let speed_x = sv_kinetic_energy::get_speed_x(weapon.lua_state_agent);
    if speed_x.abs() < 0.21 {
        weapon.clear_lua_stack();
        if speed_x < 0.0 {
            lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -0.2, 0.0);
        }
        else {
            lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.2, 0.0);
        }
        sv_kinetic_energy::set_speed(weapon.lua_state_agent);
        weapon.clear_lua_stack();
        lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy::set_accel(weapon.lua_state_agent);
    }
    0.into()
}

#[status_script(agent = "samusd_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn samusd_cshot_shoot_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    if utility::get_kind(&mut *oboma)  == *FIGHTER_KIND_SAMUSD {
        // WorkModule::off_flag(oboma, FIGHTER_SAMUSD_INSTANCE_WORK_ID_FLAG_CSHOT_ACTIVE);
        WorkModule::set_int(oboma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_CSHOT_ID);
    }
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        samusd_wait_main,
        samusd_attackair_main,
        samusd_specialn_hold_main,
        samusd_specialn_hold_exit,
        samusd_speciallw_air_main,
        samusd_cshot_shoot_init,
        samusd_cshot_shoot_exec,
        samusd_cshot_shoot_end
    );
}