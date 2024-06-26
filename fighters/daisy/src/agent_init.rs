use super::*;

extern "C" {
    #[link_name = "specials_pre_generic"]
    pub fn specials_pre_generic(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub unsafe extern "C" fn daisy_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && ItemModule::is_have_item(fighter.module_accessor, 0) {
        return 0.into();
    }
    1.into()
}

#[no_mangle]
pub unsafe extern "C" fn daisy_itemtoss_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PAD_FLAG].get_i32() == *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER {
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let special_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("special_stick_y"));
        let mut throw = false;
        if special_stick_y <= -stick_y
            // if [
            //     *ITEM_KIND_PEACHDAIKON,
            //     *ITEM_KIND_DAISYDAIKON
            // ].contains(&ItemModule::get_have_item_kind(fighter.module_accessor, 0)) {
        && ItemModule::is_have_item(fighter.module_accessor, 0) {
            throw = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].assign(&L2CValue::Ptr(daisy_itemtoss_pre as *const () as _));
    fighter.global_table[CHECK_AIR_ITEM_THROW_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_AIR_JUMP_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(specials_pre_generic as *const () as _));
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(daisy_speciallw_pre as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}
