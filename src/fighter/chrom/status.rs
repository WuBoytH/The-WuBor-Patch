use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn chrom_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_1 as u32,
        0
    );
    KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

#[status_script(agent = "chrom", status = FIGHTER_STATUS_KIND_RUN_BRAKE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn chrom_runbrake_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_RunBrake();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_RunBrake_Main as *const () as _))
}

pub fn install() {
    install_status_scripts!(
        chrom_attack_pre,
        chrom_runbrake_main
    );
}