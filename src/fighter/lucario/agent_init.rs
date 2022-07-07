use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*, table_const::*},
    super::fgc::*
};

pub unsafe extern "C" fn lucario_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MACH_VALIDITY)
    || VarModule::is_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_HI) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn lucario_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::USED_AURA_CHARGE_AIR);
    }
    if MiscModule::is_damage_check(fighter.module_accessor, false) {
        VarModule::off_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_HI);
    }
    1.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_LUCARIO {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(lucario_specialhi_pre as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(lucario_status_end_control as *const () as _));
        fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(lucario_fgc as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
