use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

pub mod special_s {
    use super::*;

    pub unsafe extern "C" fn ike_special_s_main_loop_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH {
            let cont = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && GroundModule::is_status_cliff(fighter.module_accessor) {
                fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END.into(), true.into());
                true
            }
            else {
                false
            };
            if cont {
                return 0.into();
            }
        }
        1.into()
    }
}