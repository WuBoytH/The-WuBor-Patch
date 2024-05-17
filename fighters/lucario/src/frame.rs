use super::*;
use super::helper::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn lucario_training_tools(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode()
    && [
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD
    ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            lucario_drain_aura(fighter, false);

        }
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            lucario_gain_aura(fighter);
        }
    }
}

// unsafe extern "C" fn lucario_super_dash_cancel(fighter: &mut L2CFighterCommon) {
//     if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH
//     && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
//     || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
//     && !fighter.global_table[IS_STOP].get_bool()
//     && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
//         if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//             VarModule::on_flag(fighter.module_accessor, vars::lucario::status::flag::SPECIAL_HI_SUPER_DASH_CANCEL);
//         }
//         fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END.into(), false.into());
//     }
// }

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    lucario_training_tools(fighter);
    // lucario_super_dash_cancel(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}