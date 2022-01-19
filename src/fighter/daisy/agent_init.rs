use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

pub unsafe extern "C" fn daisy_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            return 0.into();
        }
    }
    1.into()
}

pub unsafe extern "C" fn daisy_itemtoss_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PAD_FLAG].get_i32() == *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let special_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
        let mut throw = false;
        if special_stick_y <= -stick_y {
            // if [
            //     *ITEM_KIND_PEACHDAIKON,
            //     *ITEM_KIND_DAISYDAIKON
            // ].contains(&ItemModule::get_have_item_kind(fighter.module_accessor, 0)) {
            if ItemModule::is_have_item(fighter.module_accessor, 0) {
                throw = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}
