#![allow(non_snake_case)]

use super::*;

pub unsafe extern "C" fn guard_crush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
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
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn guard_crush_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessShieldBreakFly_initStatus()
}

pub unsafe extern "C" fn guard_crush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_crush_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_crush_main_loop as *const () as _))
}

pub unsafe extern "C" fn guard_crush_common(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::BURNOUT) {
        EffectModule::req_common(fighter.module_accessor, Hash40::new("burnout"), 0.0);
    }
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::BURNOUT);

    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);

    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("rebound"), true);
    if cancel_frame == 0.0 {
        cancel_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("rebound"));
    }

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("rebound"),
        0.0,
        cancel_frame / 60.0,
        false,
        0.0,
        false,
        false
    );

    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_guardbreak"), true, false, false, false, enSEType(0));

    WorkModule::set_int(fighter.module_accessor, 10, 0x11000017); // FIGHTER_STATUS_FURAFURA_STAND_WORK_INT_TERMINATE_XLU_COUNT

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHIELD_BREAK_FLY_NUM);

    let shield_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
    if shield_lr != 0.0 {
        PostureModule::set_lr(fighter.module_accessor, shield_lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            -2.1 * shield_lr,
            0.0
        );
    }

    if !StopModule::is_stop(fighter.module_accessor) {
        guard_crush_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(guard_crush_substatus as *const () as _));
}

pub unsafe extern "C" fn guard_crush_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::count_down_int(fighter.module_accessor, 0x11000017, 0) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    0.into()
}

pub unsafe extern "C" fn guard_crush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}