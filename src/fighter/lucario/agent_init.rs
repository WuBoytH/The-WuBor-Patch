use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
    super::fgc::*
};

pub unsafe extern "C" fn lucario_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::USED_AURA_CHARGE_AIR);
        if ![
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
        ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
            VarModule::off_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        }
    }
    0.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_LUCARIO {
            return;
        }
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_status_end_control as *const () as _));
        fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(lucario_fgc as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
