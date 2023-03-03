use crate::imports::status_imports::*;

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn sonic_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dash_cancel = VarModule::is_flag(fighter.battle_object, fighter::status::flag::IS_DASH_CANCEL);
    fighter.status_pre_DashCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_SONIC_DASH,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    VarModule::set_flag(fighter.battle_object, fighter::status::flag::IS_DASH_CANCEL, dash_cancel);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        sonic_dash_pre
    );
}