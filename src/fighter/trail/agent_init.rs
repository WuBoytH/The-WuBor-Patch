use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    crate::table_const::*
};

pub unsafe extern "C" fn trail_guard_cont_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::on_flag(fighter.module_accessor, 0x2100000C);
        let fighta = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        FighterSpecializer_Trail::change_magic(fighta);
    }
    false.into()
}
