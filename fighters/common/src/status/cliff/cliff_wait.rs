use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_CliffWait)]
unsafe extern "C" fn status_pre_cliffwait(fighter: &mut L2CFighterCommon) -> L2CValue {
    let (flag, int, float) = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
        (
            // *FIGHTER_STATUS_WORK_KEEP_FLAG_CLIFF_WAIT_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_CLIFF_WAIT_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_CLIFF_WAIT_FLOAT
        )
    }
    else {
        (0, 0, 0)
    };

    let succeeds = if MotionModule::motion_kind(fighter.module_accessor) == hash40("cliff_catch")
    && !MotionModule::is_end(fighter.module_accessor) {
        *FS_SUCCEEDS_KEEP_VISIBILITY
    }
    else {
        0
    };

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_CLIFF),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF,
        *GROUND_CORRECT_KIND_CLIFF as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        flag,
        int,
        float,
        succeeds
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY |
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE |
            *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT |
            *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_WARP
        ) as u32,
        0,
        0
    );

    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_get_cliff_wait_hit_xlu_frame)]
unsafe extern "C" fn get_cliff_wait_hit_xlu_frame(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_frame")
    ) as f32;
    let air_xlu_max_air_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_air_frame")
    ) as f32;
    let damage_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_frame")
    ) as f32;
    let damage_xlu_max_damage = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_damage")
    ) as f32;
    let xlu_min_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_xlu_min_frame")
    ) as f32;

    // Vanilla Ultimate's Ledge Intangibility Formula
    let air_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) as f32;
    let xlu_from_air = (air_xlu_max_frame * (air_frame / air_xlu_max_air_frame)).clamp(0.0, air_xlu_max_frame);
    // println!("xlu from air: {}", xlu_from_air);
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    let xlu_from_damage = (damage_xlu_max_frame - ((damage / damage_xlu_max_damage) * damage_xlu_max_frame)).clamp(0.0, damage_xlu_max_frame);
    // println!("xlu from damage: {}", xlu_from_damage);
    let xlu_frame = xlu_from_air + xlu_from_damage;
    let cliff_hang_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("cliff_hang_invincible_frame"), 0);
    // println!("cliff hang invincible frame? {}", cliff_hang_invincible_frame);
    // println!("final xlu: {}", (xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX));
    ((xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX)).into()
}

#[skyline::hook(replace = L2CFighterCommon_status_CliffWait_Main)]
unsafe extern "C" fn status_cliffwait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if situation == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }

    if situation == *SITUATION_KIND_AIR
    || !GroundModule::is_status_cliff(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    if sv_information::stage_id() == 0x145
    && FighterUtil::check_cliff_separated(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_DAMAGE_FALL) {
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
        return 1.into();
    }

    let cat1 = fighter.global_table[CMD_CAT1].get_i32();

    if fighter.global_table[STATUS_FRAME].get_f32() < 1.0 {
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
        && situation == *SITUATION_KIND_CLIFF {
            fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_ROBBED.into(), false.into());
            return 1.into();
        }

        return 0.into();
    }

    // Sure it's a feature now
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
    && situation == *SITUATION_KIND_CLIFF {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_ROBBED.into(), false.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP_BUTTON)
    && situation == *SITUATION_KIND_CLIFF
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_JUMP1.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP)
    && situation == *SITUATION_KIND_CLIFF
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_JUMP) {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_JUMP1.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ATTACK)
    && situation == *SITUATION_KIND_CLIFF
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_ATTACK.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_SPEICAL)
    && situation == *SITUATION_KIND_CLIFF
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_ATTACK.into(), true.into());
        return 1.into();
    }

    let cat2 = fighter.global_table[CMD_CAT2].get_i32();

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_ESCAPE)
    && situation == *SITUATION_KIND_CLIFF
    && cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_ESCAPE.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_CLIMB)
    && situation == *SITUATION_KIND_CLIFF
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_CLIMB) {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CLIMB.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
    && situation == *SITUATION_KIND_CLIFF {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_FALL)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_RELEASE) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER) {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_CLIFF_RELEASE_NUM);
            }
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }

    fighter.sub_cliff_uniq_process_main();
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if motion == hash40("cliff_catch")
    && MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("cliff_wait"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }

    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_cliffwait,
            get_cliff_wait_hit_xlu_frame,
            status_cliffwait_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}