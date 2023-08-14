use crate::imports::status_imports::*;
use crate::fighter::common::agent_inits::*;

pub unsafe extern "C" fn brave_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH
    || MiscModule::is_damage_check(fighter.module_accessor, false) {
        VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
    if status == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::set_int(fighter.battle_object, brave::instance::int::NEXT_ROLL_INDEX, 0);
        VarModule::set_int(fighter.battle_object, brave::instance::int::USED_SPELL_MASK, 0);
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_1, -1);
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_2, -1);
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_3, -1);
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_4, -1);
    }
    0.into()
}

#[event(initialize)]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_BRAVE {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(specialhi_pre_generic as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(brave_status_end_control as *const () as _));
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_1, -1);
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_2, -1);
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_3, -1);
        VarModule::set_int(fighter.battle_object, brave::instance::int::SPELL_SLOT_4, -1);
    }
}

pub fn install() {
    agent_init::install();
}
