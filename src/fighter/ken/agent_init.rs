use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*,
    super::vars::*
};

unsafe extern "C" fn ken_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE) != FIGHTER_KEN_QUICK_STEP_STATE_DISABLE {
        return 1.into();
    }
    0.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_KEN {
            return;
        }
        fighter.global_table[SPECIAL_LW_PRE].assign(&L2CValue::Ptr(ken_speciallw_pre as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
