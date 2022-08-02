use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

pub unsafe extern "C" fn demon_attack_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_GetLightItemImm(L2CValue::Void());
    (StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *STATUS_KIND_NONE).into()
}

pub unsafe extern "C" fn demon_attack_loop_common(fighter: &mut L2CFighterCommon, status: L2CValue) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(status, false.into());
            return true.into();
        }
    }
    false.into()
}
