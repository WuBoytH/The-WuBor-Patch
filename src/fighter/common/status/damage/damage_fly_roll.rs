use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFlyRoll)]
unsafe extern "C" fn status_pre_damageflyroll(fighter: &mut L2CFighterCommon) -> L2CValue {
    let disable_passive = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_THROWN_WORK_FLAG_DISABLE_PASSIVE);
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_FLY,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLOAT,
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
        *FIGHTER_STATUS_ATTR_DAMAGE as u32,
        0,
        0
    );
    VarModule::set_flag(fighter.module_accessor, damage_fly_roll::flag::DISABLE_PASSIVE, disable_passive);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyRoll_Common)]
unsafe extern "C" fn status_damageflyroll_common(fighter: &mut L2CFighterCommon) {
    MotionAnimcmdModule::call_script_single(
        fighter.module_accessor,
        *FIGHTER_ANIMCMD_EXPRESSION,
        Hash40::new_raw(0x1b19dc3bd1),
        -1
    );
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    let passives = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in passives.iter() {
        let able = if VarModule::is_flag(fighter.module_accessor, damage_fly_roll::flag::DISABLE_PASSIVE) {
            WorkModule::unable_transition_term
        }
        else {
            WorkModule::enable_transition_term
        };
        able(fighter.module_accessor, *x);
    }
    fighter.sub_DamageFlyCommon_check_dead();
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_damageflyroll,
            status_damageflyroll_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}