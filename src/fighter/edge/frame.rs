use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::{lua_const::*, *}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[line("edge", main)]
fn edge_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = fighter.global_table[STATUS_KIND].get_i32();

        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            if ![
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH,
                *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END,
                *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING,
                *FIGHTER_STATUS_KIND_JUMP_SQUAT
            ].contains(&status) {
                VarModule::set_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT, 0);
            }
        }
        if status == *FIGHTER_STATUS_KIND_JUMP_SQUAT
        || fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if VarModule::get_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT) == 0 {
                VarModule::set_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT, 1);
            }
        }

        // Blade Dash Cancels

        if status == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && VarModule::get_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT) < 2 {
                VarModule::on_flag(fighter.battle_object, edge::instance::flag::SPECIAL_HI_CANCEL);
            }
        }
        if [
            *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END,
            *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING
        ].contains(&status)
        && VarModule::is_flag(fighter.battle_object, edge::instance::flag::SPECIAL_HI_CANCEL)
        && !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
                VarModule::inc_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT);
            }
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
            || fighter.sub_transition_group_check_ground_jump().get_bool()
            || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 && fighter.sub_transition_group_check_air_jump_attack().get_bool())
            || fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                VarModule::set_int(fighter.battle_object, edge::instance::int::SPECIAL_HI_CANCEL_COUNT, 2);
            }
        }
    }
}

pub fn install() {
    edge_frame::install();
}