use crate::imports::*;
use super::super::helper::*;

unsafe extern "C" fn lucario_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    WorkModule::set_customize_no(fighter.module_accessor, 0, 0);
    // lucario_drain_aura(fighter, false);
    lucario_special_set_kinetic(fighter);
    lucario_special_n_joint_translate(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if !is_end {
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            lucario_special_set_kinetic(fighter);
            return 0.into();
        }
    }
    else {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && VarModule::get_int(fighter.module_accessor, lucario::instance::int::AURA_LEVEL) > 2 {
            lucario_drain_aura(fighter, true);
            VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB);
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                WorkModule::set_customize_no(fighter.module_accessor, 1, 0);
            }
            else {
                WorkModule::set_customize_no(fighter.module_accessor, 2, 0);
            }
        }
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, -1);
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucario_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_n_save_charge_status(fighter);
    0.into()
}

unsafe extern "C" fn lucario_special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    let ground_mot;
    let air_mot;
    if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB) {
        ground_mot = hash40("special_n_hold2");
        air_mot = hash40("special_air_n_hold2");
    }
    else {
        ground_mot = hash40("special_n_hold");
        air_mot = hash40("special_air_n_hold");
    }
    WorkModule::set_int64(fighter.module_accessor, ground_mot as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, air_mot as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::change_status(
        fighter.module_accessor,
        *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL,
        *WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE,
        ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
    );
    lucario_special_n_hold_set_kinetic(fighter);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_START_FROM_GROUND);
    }
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_hold_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_special_n_hold_set_kinetic(fighter);
        return 0.into();
    }
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    if charge >= max_charge_frame as i32 {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucario_special_n_hold_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB) {
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                ENERGY_STOP_RESET_TYPE_FREE,
                speed_x,
                speed_y.clamp(-1.0, 1.0),
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                0.06,
                0.06
            );
            sv_kinetic_energy!(
                clear_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY
            );
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}

unsafe extern "C" fn lucario_special_n_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_n_save_charge_status(fighter);
    0.into()
}

unsafe extern "C" fn lucario_special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    let ground_mot;
    let air_mot;
    if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB) {
        ground_mot = hash40("special_n_shoot2");
        air_mot = hash40("special_air_n_shoot2");
    }
    else {
        ground_mot = hash40("special_n_shoot");
        air_mot = hash40("special_air_n_shoot");
    }
    WorkModule::set_int64(fighter.module_accessor, ground_mot as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, air_mot as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    if max_charge_frame < charge as f32 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX);
    }
    lucario_special_n_shoot_set_kinetic(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB_ENABLE_FALL) {
        VarModule::off_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB_ENABLE_FALL);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            VarModule::on_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB_FALLING);
        }
    }
    if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_ENABLE_SUPERDASH) {
        let situation = fighter.global_table[SITUATION_KIND].clone();
        special_cancel_common(fighter, situation, [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI].to_vec());
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB_FALLING)
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 0.into();
            }
            lucario_special_n_shoot_set_kinetic(fighter);
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucario_special_n_shoot_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        lucario_special_air_mot_helper(fighter);
        if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB) {
            if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB_FALLING) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
    }
    else {
        lucario_special_set_ground(fighter);
        lucario_special_ground_mot_helper(fighter);
        if VarModule::is_flag(fighter.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_N, lucario_special_n_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_N, lucario_special_n_end);

    agent.status(smashline::Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD, lucario_special_n_hold_main);
    agent.status(smashline::End, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD, lucario_special_n_hold_end);

    agent.status(smashline::Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT, lucario_special_n_shoot_main);
}