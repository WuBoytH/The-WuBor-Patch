use crate::imports::*;

unsafe extern "C" fn pikachu_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn pikachu_special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let special_hi_stop_y_ = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_stop_y_"));
    WorkModule::set_int(fighter.module_accessor, special_hi_stop_y_, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_FALL_STOP_COUNTER);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_GUIDE_EFFECT_HANDLE);
    // Usually calls the effect guide, we don't call it for Quick Attack 1 anymore.
    0.into()
}

unsafe extern "C" fn pikachu_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_QUICK_ATTACK_GUIDE_EFFECT_LAST_VISIBLE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_COUNT);
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        hash40("special_air_hi_start")
    }
    else {
        hash40("special_hi_start")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if is_end {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let angle = stick_y * 20.0;
        let lr = PostureModule::lr(fighter.module_accessor);
        let special_hi_warp_spd_add_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_warp_spd_add_"));
        let x_angle = angle.to_radians().cos();
        let y_angle = angle.to_radians().sin();
        WorkModule::set_float(fighter.module_accessor, lr * special_hi_warp_spd_add_ * x_angle, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_WARP_SPEED_X);
        WorkModule::set_float(fighter.module_accessor, special_hi_warp_spd_add_ * y_angle, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_WARP_SPEED_Y);
        WorkModule::set_float(fighter.module_accessor, x_angle * lr, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_PREV_STICK_X);
        WorkModule::set_float(fighter.module_accessor, y_angle, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLOAT_QUICK_ATTACK_PREV_STICK_Y);
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP.into(), false.into());
    }
    is_end.into()
}

unsafe extern "C" fn pikachu_special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_FALL_STOP_COUNTER);
    let fall_stop_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_FALL_STOP_COUNTER);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if 0 <= fall_stop_counter {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
        else {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                set_accel,
                fighter,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.002666667
            );
        }
    }
    // Usually calls the effect guide, we don't call it for Quick Attack 1 anymore.
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_direction2"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_QUICK_ATTACK_GUIDE_EFFECT_HANDLE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_QUICK_ATTACK_GUIDE_EFFECT_LAST_VISIBLE);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_pre);
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_init);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_main);
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_exec);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_end);
}