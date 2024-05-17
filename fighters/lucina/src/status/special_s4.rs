use super::*;
use super::super::helper::*;

unsafe extern "C" fn lucina_special_s4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
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
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn lucina_special_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if spent_meter_super(fighter.module_accessor) {
        let spent = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SPENT_SP);
        let meter_max = VarModule::get_float(fighter.module_accessor, vars::yu::instance::float::SP_GAUGE_MAX);
        FGCModule::update_meter(fighter.module_accessor, -spent, meter_max, vars::yu::instance::float::SP_GAUGE);
        VarModule::set_int(fighter.module_accessor, vars::yu::instance::int::SP_FLASH_TIMER, 40);
        VarModule::on_flag(fighter.module_accessor, vars::yu::status::flag::IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_s1"),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_lightningflash_loop as *const () as _))
}

unsafe extern "C" fn lucina_lightningflash_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        if CustomCancelManager::execute_cancel(fighter) {
            return 1.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, lucina_special_s4_pre);
    agent.status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, lucina_special_s4_main);
}