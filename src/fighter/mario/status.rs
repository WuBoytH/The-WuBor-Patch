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
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    },
    super::vl
};

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attacks4_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackXX4Start();
    mario_remove_hammer(fighter);
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attacks4_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackS4Hold();
    mario_remove_hammer(fighter);
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attacks4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_attackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    mario_remove_hammer(fighter);
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn mario_landingattackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_LandingAttackAir();
    mario_remove_hammer(fighter);
    0.into()
}

unsafe extern "C" fn mario_remove_hammer(fighter: &mut L2CFighterCommon) {
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP) {
            ArticleModule::remove(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_PUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn mario_specials_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn mario_specials_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x * 0.5
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable
            );
        }
        else {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y / 1.5
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x * 0.2
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable / 2.0
            );
        }
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
        VarModule::off_flag(fighter.battle_object, mario::instance::flag::SPECIAL_LW_BLJ_PREV);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        VarModule::set_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND, mario::SPECIAL_LW_KIND_LONG_JUMP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        VarModule::off_flag(fighter.battle_object, mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        VarModule::set_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND, mario::SPECIAL_LW_KIND_GROUND_POUND);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn mario_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn mario_speciallw_shoot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    macros::SA_SET(fighter, *SITUATION_KIND_AIR);
    if VarModule::get_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND) == mario::SPECIAL_LW_KIND_LONG_JUMP {
        VarModule::off_flag(fighter.battle_object, mario::status::flag::SPECIAL_LW_BLJ);
        let dir = FGCModule::get_command_stick_direction(fighter, true);
        let speed_x : f32;
        let speed_y : f32;
        if [6, 3, 9].contains(&dir) {
            speed_x = vl::param_special_lw::long_jump_speed_strong_x;
            speed_y = vl::param_special_lw::long_jump_speed_strong_y;
            VarModule::set_int(fighter.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, mario::LONG_JUMP_S);
            VarModule::off_flag(fighter.battle_object, mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        }
        else if [4, 7, 1].contains(&dir) {
            VarModule::on_flag(fighter.battle_object, mario::status::flag::SPECIAL_LW_BLJ);
            if VarModule::is_flag(fighter.battle_object, mario::instance::flag::SPECIAL_LW_BLJ_PREV) {
                speed_x = -vl::param_special_lw::long_jump_speed_strong_x;
                speed_y = 0.0;
                VarModule::set_int(fighter.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, mario::LONG_JUMP_B);
            }
            else {
                speed_x = vl::param_special_lw::long_jump_speed_weak_x;
                speed_y = vl::param_special_lw::long_jump_speed_weak_y;
                VarModule::set_int(fighter.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, mario::LONG_JUMP_W);
            }
            VarModule::on_flag(fighter.battle_object, mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        }
        else {
            speed_x = vl::param_special_lw::long_jump_speed_mid_x;
            speed_y = vl::param_special_lw::long_jump_speed_mid_y;
                VarModule::set_int(fighter.battle_object, mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, mario::LONG_JUMP_M);
            VarModule::off_flag(fighter.battle_object, mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        }
        macros::SET_SPEED_EX(fighter, speed_x, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn mario_speciallw_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND) == mario::SPECIAL_LW_KIND_LONG_JUMP {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_mul = if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
            1.0
        }
        else {
            vl::param_special_lw::long_jump_air_accel_y_mul
        };
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * air_accel_mul;
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_accel_x * vl::param_special_lw::long_jump_air_accel_x_mul
        );
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        lua_bind::FighterKineticEnergyGravity::set_speed(
            gravity as *mut smash::app::FighterKineticEnergyGravity,
            -vl::param_special_lw::ground_pound_fall_speed
        );
        lua_bind::FighterKineticEnergyGravity::set_accel(gravity as *mut smash::app::FighterKineticEnergyGravity, 0.0);
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    if VarModule::get_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND) == mario::SPECIAL_LW_KIND_LONG_JUMP {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_light"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_longjump_jump_main_loop as *const () as _))
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_light"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_groundpound_fall_main_loop as *const () as _))
    }
}

unsafe extern "C" fn mario_speciallw_longjump_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, mario::status::flag::SPECIAL_LW_LANDING)
    && !fighter.sub_air_check_fall_common().get_bool() {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn mario_speciallw_groundpound_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_CATEGORY_MASK_ALL)
    && FGCModule::get_command_stick_direction(fighter, false) == 8 {
        VarModule::set_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND, mario::SPECIAL_LW_KIND_GROUND_POUND_CANCEL);
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND) == mario::SPECIAL_LW_KIND_LONG_JUMP {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_hold"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_longjump_end_main_loop as *const () as _))
    }
    else if VarModule::get_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND) == mario::SPECIAL_LW_KIND_GROUND_POUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_hold"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_groundpound_land_main_loop as *const () as _))
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        macros::SET_SPEED_EX(fighter, 0.0, -2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_heavy"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_groundpound_cancel_main_loop as *const () as _))
    }
}

unsafe extern "C" fn mario_speciallw_longjump_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if FGCModule::cancel_exceptions(fighter, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW, false).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mario_speciallw_groundpound_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mario_speciallw_groundpound_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        mario_attacks4_start_end,
        mario_attacks4_hold_end,
        mario_attacks4_end,
        mario_attackair_end,
        mario_landingattackair_end,
        mario_specials_init,
        mario_specials_exec,
        mario_speciallw_main,
        mario_speciallw_shoot_init,
        mario_speciallw_shoot_exec,
        mario_speciallw_shoot_main,
        mario_speciallw_charge_main
    );
}