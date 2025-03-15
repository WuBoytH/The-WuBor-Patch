#![allow(non_snake_case)]

use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_ShieldBreakFly)]
unsafe extern "C" fn status_pre_shieldbreakfly(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::BURNOUT) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, vars::fighter::status::GUARD_CRUSH);
        return 1.into();
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_AIR_S_BREAK,
        *GROUND_CORRECT_KIND_AIR as u32,
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
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_shield_break_fly_common)]
unsafe extern "C" fn sub_status_shield_break_fly_common(fighter: &mut L2CFighterCommon, arg_1: L2CValue) {
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    WorkModule::set_float(fighter.module_accessor, shield_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);

    fighter.clear_lua_stack();
    let group = sv_fighter_util::get_dead_up_camera_hit_my_distance_group(fighter.lua_state_agent);
    if FighterUtil::check_melee_rule_time(300.0, smash::app::FighterCheckMeleeRuleTime{ 0: group }, true) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("shield_break_fly"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if arg_1.get_bool() {
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_guardbreak"), true, false, false, false, enSEType(0));
    }

    let shield_break_xlu_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_break_xlu_frame"));
    WorkModule::set_int(fighter.module_accessor, shield_break_xlu_frame, 0x11000017); // FIGHTER_STATUS_FURAFURA_STAND_WORK_INT_TERMINATE_XLU_COUNT

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHIELD_BREAK_FLY_NUM);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_shieldbreakfly,
            sub_status_shield_break_fly_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}