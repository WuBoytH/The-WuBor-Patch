use super::*;

// #[skyline::hook(replace = L2CFighterCommon_status_pre_CapturePulled)]
// unsafe extern "C" fn status_pre_capturepulled(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let mut situation = SITUATION_KIND_NONE;
//     let mut correct = GROUND_CORRECT_KIND_KEEP;
//     if LinkModule::is_link(fighter.module_accessor, *LINK_NO_CAPTURE) {
//         let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *LINK_NO_CAPTURE, true) as u32;
//         if lua_bind::BattleObjectManager::is_active_find_battle_object(singletons::BattleObjectManager(), parent_id) {
//             let kind = sv_battle_object::kind(parent_id);
//             if kind == *FIGHTER_KIND_LUCARIO {
//                 let lucario_boma = sv_battle_object::module_accessor(parent_id);
//                 if [
//                     *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW
//                 ].contains(&StatusModule::status_kind(lucario_boma)) {
//                     println!("Force Palmed!");
//                     situation = SITUATION_KIND_AIR;
//                     correct = GROUND_CORRECT_KIND_NONE;
//                 }
//             }
//         }
//     }
//     fighter.status_pre_CapturePulled_common(situation.into(), correct.into())
// }

#[skyline::hook(replace = L2CFighterCommon_CapturePulledCommon)]
unsafe extern "C" fn capturepulledcommon(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_NONE, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_SITUATION);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_WAIT);
    if LinkModule::is_link(fighter.module_accessor, *LINK_NO_CAPTURE) {
        let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *LINK_NO_CAPTURE, true) as u32;
        if lua_bind::BattleObjectManager::is_active_find_battle_object(singletons::BattleObjectManager(), parent_id) {
            let kind = sv_battle_object::kind(parent_id);
            if kind == *FIGHTER_KIND_LUIGI {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_CAPTURE_PULLED_SE);
            }
            if kind == *FIGHTER_KIND_LUCARIO {
                let lucario_boma = sv_battle_object::module_accessor(parent_id);
                if [
                    *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW
                ].contains(&StatusModule::status_kind(lucario_boma)) {
                    GroundModule::set_passable_check(fighter.module_accessor, true);
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
                    && GroundModule::is_passable_ground(fighter.module_accessor) {
                        fighter.set_situation(SITUATION_KIND_AIR.into());
                    }
                }
            }
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_CapturePulledCommon_Main)]
unsafe extern "C" fn capturepulledcommon_main(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let pulled_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_SITUATION);
    if situation == pulled_sit {
        return;
    }
    let mot = if situation == *SITUATION_KIND_GROUND {
        fighter.FighterStatusCaptrue_set_correct_ground();
        WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_MOTION_KIND_GROUND)
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_AIR);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCHED_BUTTERFLYNET) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCHED_PICKEL) {
                WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_MOTION_KIND_GROUND)
            }
            else {
                let offset = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_MOTION_KIND_OFFSET);
                CaptureModule::motion(fighter.module_accessor, Hash40::new_raw(0xe4308c047), offset)
            }
        }
        else {
            WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_MOTION_KIND_GROUND)
        }
    };
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_FIRST) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_FIRST);
    }
    else {
        let frame = MotionModule::frame(fighter.module_accessor);
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            frame,
            1.0,
            0.0,
            false,
            false
        );
    }
    WorkModule::set_int(fighter.module_accessor, situation, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_INT_SITUATION);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            // status_pre_capturepulled,
            capturepulledcommon,
            capturepulledcommon_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}