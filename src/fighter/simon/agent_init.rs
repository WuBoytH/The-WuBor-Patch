use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    custom_var::*,
    smashline::*,
    wubor_utils::{vars::*, table_const::*}
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SIMON {
            return;
        }
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
    }
}

#[fighter_init]
fn fighter_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SIMON {
            return;
        }
        VarModule::set_int(fighter.battle_object, richter::instance::int::AXE_ID, *BATTLE_OBJECT_ID_INVALID);
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
    install_agent_init_callbacks!(
        fighter_init
    );
}
