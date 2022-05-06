use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    wubor_utils::table_const::*,
    custom_status::*,
    super::{
        helper::*,
        super::{vars::*, vl}
    }
};

#[status_script(agent = "marth", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn marth_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    marth_speciallw_common_end(fighter);
    marth_stance_common_end(fighter);
    0.into()
}

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_speciallw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    macros::PLAY_SE(fighter, Hash40::new("se_marth_special_l02"));
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_hit_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            marth_speciallw_hit_mot_helper(fighter);
        }
    }
    let frame = fighter.global_table[MOTION_FRAME].get_f32();
    if frame > 1.0
    && !CancelModule::is_enable_cancel(fighter.module_accessor) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    if frame > 10.0
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if marth_stance_cancel_helper(fighter).get_bool()
        || marth_stance_dash_cancel_helper(fighter, true).get_bool() {
            return 1.into();
        }
    }
    marth_stance_mot_end_helper(fighter);
    0.into()
}

unsafe extern "C" fn marth_speciallw_hit_mot_helper(fighter: &mut L2CFighterCommon) {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let correct;
    let mot;
    if sit != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = Hash40::new("special_air_lw");
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = Hash40::new("special_lw");
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
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

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn marth_speciallw_hit_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU)
    && ![
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3),
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    marth_speciallw_common_end(fighter);
    marth_stance_common_end(fighter);
    0.into()
}

unsafe extern "C" fn marth_speciallw_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK) {
        ShieldModule::set_status(
            fighter.module_accessor,
            0,
            ShieldStatus(*SHIELD_STATUS_NONE),
            *FIGHTER_MARTH_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD
        );
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S

unsafe extern "C" fn marth_speciallw_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_FLAG_DISABLE_STANCE_CHANGE);
    let lr = PostureModule::lr(fighter.module_accessor);
    let turn = FighterControlModuleImpl::get_special_s_turn(fighter.module_accessor) as i32;
    if (lr < 0.0 && turn == *FIGHTER_COMMAND_TURN_LR_RIGHT) || (lr >= 0.0 && turn == *FIGHTER_COMMAND_TURN_LR_LEFT) {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_special_air_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x * vl::param_stance::special_s_start_x_mul,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y * vl::param_stance::special_s_start_y_mul
        );
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y * vl::param_stance::special_s_start_y_mul
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_special_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            speed_x * vl::param_stance::special_s_start_x_mul,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_lw_special_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_lw_special_air_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_FLASHING_BLADE) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_START);
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_DASH) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S_DASH);
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S_DASH

unsafe extern "C" fn marth_speciallw_specials_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_specials_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let mut angle : f32;
        let stick_y_threshold = 0.5;
        if stick_y >= stick_y_threshold {
            angle = 15.0;
        }
        else if stick_y <= -stick_y_threshold {
            angle = -15.0;
        }
        else {
            angle = 0.0;
        }
        WorkModule::set_float(fighter.module_accessor, angle, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        if PostureModule::lr(fighter.module_accessor) < 0.0 {
            angle *= -1.0;
        }
        angle = angle.to_radians();
        sv_kinetic_energy!(
            set_angle,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            angle
        );
        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.9
        );
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials_dash_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_specials_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_lw_special_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_lw_special_air_s"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_END) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S_END);
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S_END

unsafe extern "C" fn marth_speciallw_specials_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_specials_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 14.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x * 0.3,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials_end_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_specials_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_START

unsafe extern "C" fn marth_speciallw_specials2_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_specials2_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_FLAG_DISABLE_STANCE_CHANGE);
    WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_LOOP_COUNT);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_START_SITUAITON);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_special_air_s2_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y * 1.5
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            vl::param_stance::special_s2_start_y
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::special_s2_start_x * lr,
            0.0
        );
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            0.0
        );
        sv_kinetic_energy!(
            mul_x_speed_max,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_START_SITUAITON);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_special_s2_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.clear_lua_stack();
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
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials2_start_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_specials2_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_LOOP);
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_LOOP

unsafe extern "C" fn marth_speciallw_specials2_loop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_specials2_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_FLAG_DISABLE_STANCE_CHANGE);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_START_SITUAITON);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_special_air_s2_loop"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::special_s2_start_x * lr,
            0.0
        );
        sv_kinetic_energy!(
            mul_x_accel_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::special_s2_x_accel_mul_air
        );
        sv_kinetic_energy!(
            mul_x_accel_add,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::special_s2_x_accel_mul_air
        );
        let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_speed_y_stable * vl::param_stance::special_s2_y_stable_mul
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_START_SITUAITON);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_special_s2_loop"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials2_loop_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_specials2_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let started_ground = WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_START_SITUAITON) == *SITUATION_KIND_GROUND;
    let loop_count = WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_LOOP_COUNT);
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if started_ground {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        else {
            // let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_LANDING);
            // fighter.change_status(status.into(), false.into());
            WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    if started_ground
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_FLAG_FINAL_BLOW);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_FLAG_FINAL_BLOW) {
            let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_END2);
            fighter.change_status(status.into(), true.into());
            return 1.into();
            // placeholder
        }
        if started_ground {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            && loop_count < vl::param_stance::special_s2_ground_loop_max {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_lw_special_s2_loop"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                let motion_mul = 1.0 - (loop_count as f32 * 0.18);
                sv_kinetic_energy!(
                    set_speed_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    motion_mul
                );
                WorkModule::inc_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_LOOP_COUNT);
            }
            else {
                let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_END);
                fighter.change_status(status.into(), false.into());
            }
        }
        else {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            && loop_count < vl::param_stance::special_s2_air_loop_max {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_lw_special_air_s2_loop"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::inc_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_LOOP_COUNT);
            }
            else {
                let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_END);
                fighter.change_status(status.into(), false.into());
            }
        }
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_END

unsafe extern "C" fn marth_speciallw_specials2_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_specials2_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_FLAG_DISABLE_STANCE_CHANGE);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_special_air_s2_end"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        sv_kinetic_energy!(
            mul_x_accel_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::special_s2_x_accel_mul_air
        );
        sv_kinetic_energy!(
            mul_x_accel_add,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::special_s2_x_accel_mul_air
        );
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y * vl::param_stance::special_s2_y_accel_mul_fall
        );
        let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_speed_y_stable * vl::param_stance::special_s2_y_stable_mul
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("run_brake_r"),
            0.0,
            2.0,
            false,
            0.0,
            false,
            false
        );
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials2_end_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_specials2_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let started_ground = WorkModule::get_int(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S2_START_SITUAITON) == *SITUATION_KIND_GROUND;
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if started_ground {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if started_ground {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
            // placeholder
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_END2

unsafe extern "C" fn marth_speciallw_specials2_end2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_specials2_end2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_FLAG_DISABLE_STANCE_CHANGE);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_special_s2_end2"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials2_end2_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_specials2_end2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_LANDING

// unsafe extern "C" fn marth_speciallw_specials2_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     StatusModule::init_settings(
//         fighter.module_accessor,
//         SituationKind(*SITUATION_KIND_GROUND),
//         *FIGHTER_KINETIC_TYPE_MOTION,
//         *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
//         GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
//         true,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
//         0
//     );
//     FighterStatusModuleImpl::set_fighter_status_data(
//         fighter.module_accessor,
//         false,
//         *FIGHTER_TREADED_KIND_NO_REAC,
//         false,
//         false,
//         false,
//         (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
//             | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
//         0,
//         *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
//         0
//     );
//     0.into()
// }

// unsafe extern "C" fn marth_speciallw_specials2_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
//     WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_FLAG_DISABLE_STANCE_CHANGE);
//     MotionModule::change_motion(
//         fighter.module_accessor,
//         Hash40::new("special_lw_special_s2_landing"),
//         0.0,
//         1.0,
//         false,
//         0.0,
//         false,
//         false
//     );
//     fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_specials2_landing_main_loop as *const () as _))
// }

// unsafe extern "C" fn marth_speciallw_specials2_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if CancelModule::is_enable_cancel(fighter.module_accessor) {
//         if fighter.sub_wait_ground_check_common(false.into()).get_bool()
//         || fighter.sub_air_check_fall_common().get_bool() {
//             return 1.into();
//         }
//     }
//     if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
//         fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
//     }
//     if MotionModule::is_end(fighter.module_accessor) {
//         fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
//     }
//     0.into()
// }

pub fn install() {
    install_status_scripts!(
        marth_speciallw_end,
        marth_speciallw_hit_main,
        marth_speciallw_hit_end
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S,
        StatusInfo::new()
            .with_pre(marth_speciallw_specials_pre)
            .with_main(marth_speciallw_specials_main)
            .with_end(marth_stance_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S_DASH,
        StatusInfo::new()
            .with_pre(marth_speciallw_specials_dash_pre)
            .with_main(marth_speciallw_specials_dash_main)
            .with_end(marth_stance_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S_END,
        StatusInfo::new()
            .with_pre(marth_speciallw_specials_end_pre)
            .with_main(marth_speciallw_specials_end_main)
            .with_end(marth_stance_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_START,
        StatusInfo::new()
            .with_pre(marth_speciallw_specials2_start_pre)
            .with_main(marth_speciallw_specials2_start_main)
            .with_end(marth_stance_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_LOOP,
        StatusInfo::new()
            .with_pre(marth_speciallw_specials2_loop_pre)
            .with_main(marth_speciallw_specials2_loop_main)
            .with_end(marth_stance_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_END,
        StatusInfo::new()
            .with_pre(marth_speciallw_specials2_end_pre)
            .with_main(marth_speciallw_specials2_end_main)
            .with_end(marth_stance_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SPECIAL_S2_END2,
        StatusInfo::new()
            .with_pre(marth_speciallw_specials2_end2_pre)
            .with_main(marth_speciallw_specials2_end2_main)
            .with_end(marth_stance_common_end)
    );
}