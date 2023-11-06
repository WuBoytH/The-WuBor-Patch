use crate::imports::status_imports::*;

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn sonic_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Throw_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_throw_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if mot == hash40("throw_f") {
        if VarModule::is_flag(fighter.module_accessor, fighter::status::flag::DASH_CANCEL) {
            if FGCModule::cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_DASH, false).get_bool() {
                VarModule::on_flag(fighter.module_accessor, fighter::status::flag::IS_DASH_CANCEL);
                return 1.into();
            }
        }
    }
    fighter.status_Throw_Main()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_THROW, sonic_throw_main);

    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL, sonic_screw_jump_aerial_main);
}