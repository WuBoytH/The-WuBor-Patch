use crate::imports::status_imports::*;

#[status_script(agent = "demon", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn demon_landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let whiff = VarModule::is_flag(fighter.battle_object, attack_air::flag::WHIFF);
    let enable_landing_attack = VarModule::is_flag(fighter.battle_object, attack_air::flag::ENABLE_LANDING_ATTACK);
    let fs_succeds = if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        *FS_SUCCEEDS_KEEP_ATTACK
    }
    else {
        0
    };
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
        fs_succeds
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
    VarModule::set_flag(fighter.battle_object, attack_air::flag::WHIFF, whiff);
    VarModule::set_flag(fighter.battle_object, attack_air::flag::ENABLE_LANDING_ATTACK, enable_landing_attack);
    0.into()
}

#[status_script(agent = "demon", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn demon_landing_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingAttackAirSub();
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_landing_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn demon_landing_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor)
    && MotionModule::motion_kind(fighter.module_accessor) == hash40("landing_air_lw") {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_FLAG_UP);
        ControlModule::reset_down_stand_fb_kind(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
        return 1.into();
    }
    fighter.status_LandingAttackAir_Main()
}

pub fn install() {
    install_status_scripts!(
        demon_landing_attack_air_pre,
        demon_landing_attack_air_main
    );
}