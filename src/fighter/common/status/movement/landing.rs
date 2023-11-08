use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Landing_param)]
unsafe extern "C" fn status_pre_landing_param(fighter: &mut L2CFighterCommon, param_1: L2CValue,  param_2: L2CValue,  param_3: L2CValue,  param_4: L2CValue,  param_5: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        param_4.get_i32(),
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        param_1.get_i32(),
        param_2.get_i32(),
        param_3.get_i32(),
        param_5.get_i32()
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_LandingLight_param)]
unsafe extern "C" fn status_pre_landinglight_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue, param_5: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        param_4.get_i32(),
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        param_1.get_i32(),
        param_2.get_i32(),
        param_3.get_i32(),
        param_5.get_i32()
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_landing_fall_special_common)]
unsafe extern "C" fn status_pre_landing_fall_special_common(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        fighter.sub_pre_landing_kinetic_type().get_i32(),
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        param_1.get_i32(),
        param_2.get_i32(),
        param_3.get_i32(),
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_landing_uniq_process_init_main_param)]
unsafe extern "C" fn sub_landing_uniq_process_init_main_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue) {
    let landing_mot = param_3.get_u64();
    let status_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if status_interrupt != param_2.get_i32()
    && status_interrupt != *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT {
        if status_interrupt != param_4.get_i32() {
            if status_interrupt == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
                let aerial_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
                let landing_lag_param;
                let landing_mot;
                let kind;
                if aerial_mot == hash40("attack_air_n") {
                    landing_lag_param = hash40("landing_attack_air_frame_n");
                    landing_mot = hash40("landing_air_n");
                    kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N;
                }
                else if aerial_mot == hash40("attack_air_f") {
                    landing_lag_param = hash40("landing_attack_air_frame_f");
                    landing_mot = hash40("landing_air_f");
                    kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
                }
                else if aerial_mot == hash40("attack_air_b") {
                    landing_lag_param = hash40("landing_attack_air_frame_b");
                    landing_mot = hash40("landing_air_b");
                    kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B;
                }
                else if aerial_mot == hash40("attack_air_hi") {
                    landing_lag_param = hash40("landing_attack_air_frame_hi");
                    landing_mot = hash40("landing_air_hi");
                    kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI;
                }
                else {
                    landing_lag_param = hash40("landing_attack_air_frame_lw");
                    landing_mot = hash40("landing_air_lw");
                    kind = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW;
                }
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, kind);
                fighter.sub_landing_attack_air_init(landing_mot.into(), landing_lag_param.into(), 0.into());
            }
            if status_interrupt == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
                fighter.sub_landing_fall_special_init(param_1);
            }
        }
    }
    else {
        if status_interrupt == *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT {
            effect!(fighter, MA_MSC_EFFECT_SET_DISABLE_LANDING_EFFECT, true);
        }
        let mut rate = 1.0;
        let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status) {
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            if mot == hash40("escape_air_slide")
            || {
                let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("invalid"), true);
                cancel_frame <= 0.0
            }
            || !FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
                let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                if landing_frame != 0.0 {
                    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(landing_mot));
                    rate = fighter.sub_calc_landing_motion_rate(end_frame.into(), landing_frame.into()).get_f32();
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_SAVING_DAMAGE_AIR].contains(&prev_status) {
            let landing_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(landing_mot));
            if end_frame < landing_frame {
                rate = fighter.sub_calc_landing_motion_rate(end_frame.into(), landing_frame.into()).get_f32();
            }
        }
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(landing_mot),
            0.0,
            rate,
            false,
            0.0,
            false,
            false
        );
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_RUN_FALL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    }

    // Landing Turn stuff which is now gone
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_LandingSub)]
unsafe extern "C" fn status_landingsub(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
    && ItemModule::get_have_item_kind(fighter.module_accessor, 0) != *ITEM_KIND_ASSIST {
        fighter.sub_landing_ground_check_common_pre();
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_landing_uniq_check();
        }
        fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_landing_uniq_check as *const () as _));
    }
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor).abs();
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor).abs();
        if stick_y < 0.00001 {
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        }
        if stick_x < 0.00001 {
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_landing_param,
            status_pre_landinglight_param,
            status_pre_landing_fall_special_common,
            sub_landing_uniq_process_init_main_param,
            status_landingsub
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}