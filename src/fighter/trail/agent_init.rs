use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    wubor_utils::table_const::*
};

unsafe extern "C" fn trail_guard_cont_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::on_flag(fighter.module_accessor, 0x2100000C);
        let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        FighterSpecializer_Trail::change_magic(fighta);
    }
    false.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_TRAIL {
            return;
        }
        fighter.global_table[GUARD_CONT_PRE].assign(&L2CValue::Ptr(trail_guard_cont_pre as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
