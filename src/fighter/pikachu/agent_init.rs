use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*},
};

pub unsafe extern "C" fn pikachu_special_hi_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn pikachu_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_LW) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn pikachu_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
        VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_LW);
    }
    0.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_PIKACHU {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(pikachu_special_hi_uniq as *const () as _));
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(pikachu_special_lw_uniq as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(pikachu_status_end_control as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
