use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    custom_var::*,
    custom_cancel::*,
    smashline::*,
    crate::fighter::common::agent_inits::*,
    wubor_utils::{vars::*, table_const::*},
    super::fgc
};

unsafe extern "C" fn richter_disable_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object_id = VarModule::get_int(fighter.battle_object, richter::instance::int::AXE_ID) as u32;
    if sv_battle_object::is_active(object_id) {
        return false.into();
    }
    true.into()
}

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_RICHTER {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(richter_disable_special_n as *const () as _));
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(specialhi_pre_generic as *const () as _));
        fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Bool(false));
        fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&L2CValue::Bool(false));
        fgc::install();
    }
}

#[fighter_init]
fn fighter_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_RICHTER {
            return;
        }
        VarModule::set_int(fighter.battle_object, richter::instance::int::AXE_ID, *BATTLE_OBJECT_ID_INVALID);
    }
}

pub fn install() {
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_richter"));
    install_agent_resets!(
        agent_reset
    );
    install_agent_init_callbacks!(
        fighter_init
    );
}
