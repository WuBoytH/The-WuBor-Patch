use {
    crate::imports::status_imports::*,
    super::super::helper::*
};

unsafe extern "C" fn ryu_attack_lw4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
        VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    ryu_attack_reset(fighter);
    fighter.status_AttackLw4Start_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_lw4_start_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_attack_lw4_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        let status_frame = fighter.global_table[STATUS_FRAME].get_f32();
        let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
        if status_frame <= attack_start_cancel_frame {
            if ryu_kara_cancel(fighter).get_bool() {
                return 1.into();
            }
        }
    }
    fighter.status_AttackLw4Start_Main()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, ryu_attack_lw4_start_main);
}