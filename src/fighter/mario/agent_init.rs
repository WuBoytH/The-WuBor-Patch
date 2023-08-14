use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

unsafe extern "C" fn mario_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, mario::instance::int::SPECIAL_LW_KIND) == mario::SPECIAL_LW_KIND_GROUND_POUND_CANCEL {
        return 0.into();
    }
    1.into()
}

#[event(start)]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_MARIO {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(mario_speciallw_pre as *const () as _));
    }
}

pub fn install() {
    agent_reset::install();
}
