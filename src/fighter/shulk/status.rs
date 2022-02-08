use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*
};

#[status_script(agent = "shulk", status = FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn shulk_speciallw_hit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG;
    let mut int = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT;
    let mut float = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT;
    let mut correct = *GROUND_CORRECT_KIND_KEEP;
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        flag = 0;
        int = 0;
        float = 0;
        correct = *GROUND_CORRECT_KIND_GROUND;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        correct as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        flag,
        int,
        float,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        0,
        0
    );
    0.into()
}

pub fn install() {
    install_status_scripts!(
        shulk_speciallw_hit_pre
    );
}