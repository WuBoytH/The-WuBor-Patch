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
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::USED_AURA_CHARGE_AIR);
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::EXTREME_SPEED_FORCE_NO_AURA);
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_S);
        if ![
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
        ].contains(&status) {
            VarModule::off_flag(fighter.battle_object, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        }
    }
    0.into()
}

pub unsafe extern "C" fn lucario_special_s_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!VarModule::is_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_S)).into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_LUCARIO {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(lucario_special_s_uniq as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_status_end_control as *const () as _));
        fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(lucario_fgc as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
