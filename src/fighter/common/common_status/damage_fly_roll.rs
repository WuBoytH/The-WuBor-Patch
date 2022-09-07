use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        phx::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyRoll_Common)]
unsafe fn status_damageflyroll_common(fighter: &mut L2CFighterCommon) {
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
    let unables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in unables.iter() {
        WorkModule::unable_transition_term(fighter.module_accessor, *x);
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
            status_damageflyroll_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}