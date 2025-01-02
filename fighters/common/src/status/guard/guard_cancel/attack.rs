use super::*;

pub unsafe extern "C" fn guard_cancel_attack_pre_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    0.into()
}

#[no_mangle]
pub unsafe extern "C" fn guard_cancel_attack_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_cancel_attack_common(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(guard_cancel_attack_main_loop_common as *const () as _))
}

#[no_mangle]
pub unsafe extern "C" fn guard_cancel_attack_common(fighter: &mut L2CFighterCommon) {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);

    HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    add_shield_health(fighter, -0.2);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("guard_cancel_attack"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

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

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE, fighter.module_accessor);

}

#[no_mangle]
pub unsafe extern "C" fn guard_cancel_attack_main_loop_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}