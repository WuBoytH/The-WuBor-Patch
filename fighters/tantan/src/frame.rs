use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

// unsafe extern "C" fn tantan_acd(fighter: &mut L2CFighterCommon) {
//     // Jabs and Tilts cancel into roll on hit or block
//     // (Potentially) half distance when canceled into
//     // Normal intangibility

//     // Status system
//     // Every specific action has a unique status kind (most of the time)
//     // ex richter bounce on hit is still considered an aerial
//     // but greninja's just goes straight into double jump
//     // ??????????

//     // Check what status we're in
//     // either through StatusModule::status_kind(fighter.module_accessor);
//     // OR
//     // fighter.global_table[0xB].get_i32()

//     let status = fighter.global_table[0xB].get_i32();

//     // check status
//     if [
//         *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, // Min Min's unique Jab status
//     ].contains(&status) {
//         // Now we check if we hit something
//         // and check if we're not in hitlag
//         if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
//         && !fighter.global_table[IS_STOP].get_bool() {
//             let escape = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
//             let escape_f = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);

//             WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
//             WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
//             fighter.sub_transition_group_check_ground_escape();

//             if !escape {
//                 WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
//             }
//             if !escape_f {
//                 WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
//             }
//         }
//     }
// }

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    // tantan_acd(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}