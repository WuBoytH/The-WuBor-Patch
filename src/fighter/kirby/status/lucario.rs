use crate::imports::status_imports::*;
use crate::fighter::lucario::helper::*;

unsafe extern "C" fn kirby_lucario_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    WorkModule::set_customize_no(fighter.module_accessor, 0, 0);
    kirby_lucario_special_set_kinetic(fighter);
    lucario_special_n_joint_translate(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_lucario_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_lucario_special_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        lucario_special_set_air(fighter);
        kirby_lucario_special_air_mot(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        lucario_special_set_ground(fighter);
        kirby_lucario_special_ground_mot(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}

unsafe extern "C" fn kirby_lucario_special_ground_mot(fighter: &mut L2CFighterCommon) {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        FighterMotionModuleImpl::change_motion_inherit_frame_keep_rate_kirby_copy(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0
        );
    }
    else {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
}

unsafe extern "C" fn kirby_lucario_special_air_mot(fighter: &mut L2CFighterCommon) {
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        FighterMotionModuleImpl::change_motion_inherit_frame_keep_rate_kirby_copy(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0
        );
    }
    else {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
}

unsafe extern "C" fn kirby_lucario_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if !is_end {
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            lucario_special_set_kinetic(fighter);
            return 0.into();
        }
    }
    else {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_lucario_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_n_save_charge_status(fighter);
    0.into()
}

unsafe extern "C" fn kirby_lucario_special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::change_status(
        fighter.module_accessor,
        *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL,
        *WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE,
        ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
    );
    kirby_lucario_special_set_kinetic(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_lucario_special_n_hold_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_lucario_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_special_set_kinetic(fighter);
        return 0.into();
    }
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    let charge = fighter.global_table[STATUS_FRAME].get_f32();
    if charge >= max_charge_frame + 4.0 {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn kirby_lucario_special_n_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_n_save_charge_status(fighter);
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N, kirby_lucario_special_n_main);
    agent.status(smashline::End, *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N, kirby_lucario_special_n_end);

    agent.status(smashline::Main, *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD, kirby_lucario_special_n_hold_main);
    agent.status(smashline::End, *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD, kirby_lucario_special_n_hold_end);
}