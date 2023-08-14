use crate::imports::status_imports::*;

#[status("pikmin", FIGHTER_STATUS_KIND_ATTACK_HI3)]
unsafe fn pikmin_attackhi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_HI3 |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

#[status("pikmin", FIGHTER_STATUS_KIND_ATTACK_HI3)]
unsafe fn pikmin_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackHi3_Common(Hash40::new("attack_hi3").into(), Hash40::new("attack_hi3").into());
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    VarModule::on_flag(fighter.battle_object, pikmin::instance::flag::ATTACK_HI3_LANDING);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    let attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_FRAME);
    let item_catch_frame_attack_3 = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("item_catch_frame_attack_3"));
    if attack_frame <= item_catch_frame_attack_3 {
        fighter.sub_GetLightItemImm(FIGHTER_STATUS_KIND_CATCH_WAIT.into());
        if StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *STATUS_KIND_NONE {
            return 0.into();
        }
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        pikmin_attackhi3_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(pikmin_attackhi3_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(pikmin_attackhi3_main_loop as *const () as _))
}

unsafe extern "C" fn pikmin_attackhi3_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK)
        || WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) != 0 {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_FRAME);
    }
    0.into()
}

unsafe extern "C" fn pikmin_attackhi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, pikmin::status::flag::ATTACK_HI3_DRIFT) {
        VarModule::off_flag(fighter.battle_object, pikmin::status::flag::ATTACK_HI3_DRIFT);
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
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    let trans = MotionModule::trans_move_speed(fighter.module_accessor);
    if trans.y < -0.001 {
        if fighter.sub_transition_group_check_air_landing().get_bool() {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
            return 0.into();
        }
    }
    if fighter.status_AttackHi3_Main_minjump().get_bool() {
        VarModule::off_flag(fighter.battle_object, pikmin::instance::flag::ATTACK_HI3_LANDING);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT);
    }
    0.into()
}

pub fn install() {
    pikmin_attackhi3_pre::install();
    pikmin_attackhi3_main::install();
}