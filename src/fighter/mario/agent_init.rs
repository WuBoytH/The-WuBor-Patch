use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::WorkModule, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    super::{fgc::*, vars::*}
};

unsafe extern "C" fn mario_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND) == FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND_CANCEL {
        return 0.into();
    }
    1.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_MARIO {
            return;
        }
        fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(mario_speciallw_pre as *const () as _));
        fighter.global_table["fgc_func"].assign(&L2CValue::Ptr(mario_fgc as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
