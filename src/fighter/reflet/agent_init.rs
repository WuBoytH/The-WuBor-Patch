use crate::imports::status_imports::*;
use crate::fighter::common::agent_inits::*;

pub unsafe extern "C" fn reflet_status_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
    || status == *FIGHTER_STATUS_KIND_REBIRTH
    || MiscModule::is_damage_check(fighter.module_accessor, false) {
        VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
    0.into()
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_REFLET {
            return;
        }
        fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(specialhi_pre_generic as *const () as _));
        fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(reflet_status_end_control as *const () as _));
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
