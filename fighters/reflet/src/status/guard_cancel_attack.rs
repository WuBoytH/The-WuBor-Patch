use super::*;

extern "C" {
    #[link_name = "guard_cancel_attack_main_common"]
    fn guard_cancel_attack_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn guard_cancel_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    FighterSpecializer_Reflet::change_hud_kind(fighter.battle_object as *mut Fighter, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
        let current_point = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
        if current_point <= 0 {
            let fightermoduleaccessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
            FighterSpecializer_Reflet::set_flag_to_table(
                fightermoduleaccessor,
                *FIGHTER_REFLET_MAGIC_KIND_SWORD,
                true,
                *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE
            );
        }
    }
    guard_cancel_attack_main_common(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);
}