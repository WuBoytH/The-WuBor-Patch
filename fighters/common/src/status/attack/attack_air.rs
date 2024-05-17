use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_uniq_process_init)]
unsafe extern "C" fn sub_attack_air_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield_stiff_mul_attack_air = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_mul_attack_air"));
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, shield_stiff_mul_attack_air);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_common)]
unsafe extern "C" fn sub_attack_air_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.attack_air_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_air_uniq as *const () as _));
    if param_1.get_bool() {
        fighter.sub_attack_air_kind();
    }
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe extern "C" fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.attack_air_common_strans().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            if CustomCancelManager::execute_cancel(fighter) {
                return true.into();
            }
            return false.into();
        }
        else {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if !MotionModule::is_end(fighter.module_accessor) {
                    return false.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackAir)]
unsafe extern "C" fn bind_address_call_status_end_attackair(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_AttackAir()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackAir)]
unsafe extern "C" fn status_end_attackair(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        FGCModule::reset_used_aerials(fighter);
    }
    fighter.status_end_Jump();
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_LandingAttackAir)]
unsafe extern "C" fn status_pre_landingattackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_landing_attack_air_init)]
unsafe extern "C" fn sub_landing_attack_air_init(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    let mot = param_1.get_int();
    let landing_lag = WorkModule::get_param_float(fighter.module_accessor, param_2.get_int(), 0) + param_3.get_f32();
    let mut motion_rate = 1.0;
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        motion_rate,
        false,
        0.0,
        false,
        false
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack_air_uniq_process_init,
            sub_attack_air_common,
            status_attackair_main_common,
            bind_address_call_status_end_attackair,
            status_end_attackair,
            status_pre_landingattackair,
            sub_landing_attack_air_init
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}