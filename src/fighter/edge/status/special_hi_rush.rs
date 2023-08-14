use crate::imports::status_imports::*;

#[status("edge", FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)]
unsafe fn edge_special_hi_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_AIR_BRAKE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), // Was ALWAYS_BOTH_SIDES
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        true,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

#[status("edge", FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)]
unsafe fn edge_special_hi_rush_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, edge::status::flag::SPECIAL_HI_CANCEL);
    }
    original(fighter)
}

pub fn install() {
    edge_special_hi_rush_pre::install();
    edge_special_hi_rush_end::install();
}