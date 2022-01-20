use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_MIIFIGHTER )]
fn miifighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC)
        && !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let status = StatusModule::status_kind(fighter.module_accessor);
            let mut cont = true;
            if status == *FIGHTER_STATUS_KIND_ATTACK
            || status == *FIGHTER_STATUS_KIND_ITEM_SWING {
                FGCModule::disable_ground_normal(fighter, ATTACK_N_MASK);
            }
            else if status == *FIGHTER_STATUS_KIND_ATTACK_S3
            || status == *FIGHTER_STATUS_KIND_ITEM_SWING_S3 {
                FGCModule::disable_ground_normal(fighter, ATTACK_S3_MASK);
            }
            else if status == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                FGCModule::disable_ground_normal(fighter, ATTACK_HI3_MASK);
            }
            else if status == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
                FGCModule::disable_ground_normal(fighter, ATTACK_LW3_MASK);
            }
            else if status == *FIGHTER_STATUS_KIND_ATTACK_S4_START
            || status == *FIGHTER_STATUS_KIND_ATTACK_S4
            || status == *FIGHTER_STATUS_KIND_ITEM_SWING_S4_START {
                FGCModule::disable_ground_normal(fighter, ATTACK_S4_MASK);
            }
            else if status == *FIGHTER_STATUS_KIND_ATTACK_HI4_START
            || status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                FGCModule::disable_ground_normal(fighter, ATTACK_HI4_MASK);
            }
            else if status == *FIGHTER_STATUS_KIND_ATTACK_LW4_START
            || status == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
                FGCModule::disable_ground_normal(fighter, ATTACK_LW4_MASK);
            }
            else {
                cont = false;
            }
            if cont {
                let normal_cancels = [
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
                ].to_vec();
                FGCModule::cancel_system(fighter, normal_cancels, [].to_vec(), false, 0);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        miifighter_frame
    );
}