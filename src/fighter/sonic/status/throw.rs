use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn sonic_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Throw_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_throw_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if mot == hash40("throw_f") {
        if VarModule::is_flag(fighter.battle_object, commons::status::flag::DASH_CANCEL) {
            if FGCModule::cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH, false).get_bool() {
                VarModule::on_flag(fighter.battle_object, commons::status::flag::IS_DASH_CANCEL);
                return 1.into();
            }
        }
    }
    fighter.status_Throw_Main()
}

pub fn install() {
    install_status_scripts!(
        sonic_throw_main
    );
}