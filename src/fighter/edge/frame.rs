use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::table_const::*,
    super::vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
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
                WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL_COUNT);
            }
        }
        if status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL_COUNT) == 0 {
                WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL_COUNT);
            }
        }
        if status == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL);
        }

        // Blade Dash Cancels

        if status == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL);
            }
        }
        if [
            *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END,
            *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING
        ].contains(&status)
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL)
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL_COUNT) < 2 {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
                WorkModule::inc_int(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL_COUNT);
            }
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
            || fighter.sub_transition_group_check_ground_jump().get_bool()
            || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 && fighter.sub_transition_group_check_air_jump_attack().get_bool())
            || fighter.sub_transition_group_check_air_jump_aerial().get_bool() {
                WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL_COUNT);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        edge_frame
    );
}