use crate::imports::status_imports::*;

#[status_script(agent = "richter", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn richter_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        richter_attack_mtrans(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(richter_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

unsafe extern "C" fn richter_attack_mtrans(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    fighter.check_attack_mtrans();
    if mot == hash40("attack_11") && mot != MotionModule::motion_kind(fighter.module_accessor)
    && VarModule::is_flag(fighter.battle_object, richter::status::flag::ATTACK_JUST_INPUT) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_12_f"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        VarModule::off_flag(fighter.battle_object, richter::status::flag::ATTACK_JUST_INPUT);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        richter_attack_main
    );
}