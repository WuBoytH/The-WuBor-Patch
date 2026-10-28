use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_fighter_pre_end_status)]
unsafe extern "C" fn sub_fighter_pre_end_status(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
        let some = fighter.global_table[0x24].get_f32();
        if some <= 2.0 {
            let status = fighter.global_table[STATUS_KIND].get_i32();
            // fighter.clear_lua_stack();
            // lua_args!(fighter, status);
            // if sv_fighter_util::is_attack_air_status(fighter.lua_state_agent, status)
            // || [
            if [
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_STATUS_KIND_ESCAPE_AIR, // new
                *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE // new
            ].contains(&status) {
                let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -air_speed_y_stable
                );
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }

    // <WuBor>
    // Handles removing ledge intangibility when not falling or double jumping.
    // This makes a big assumption where if you ledge drop, you will have had ledge intangibility.
    // May cause unintended side effects but the logic should be pretty air-tight.

    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::LEDGE_INTANGIBILITY) {
        let kind = fighter.global_table[KIND].get_i32();
        if ![
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_FLY,
            *FIGHTER_STATUS_KIND_ATTACK_AIR
        ].contains(&status)
        && !(kind == *FIGHTER_KIND_BAYONETTA && status == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F)
        && !(
            kind == *FIGHTER_KIND_PICKEL && [
                *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP,
                *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_AERIAL,
                *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL,
                *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL,
                *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START,
                *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_LOOP,
                *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_END
            ].contains(&status)
        )
        && !(kind == *FIGHTER_KIND_TRAIL && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F].contains(&status)) {
            VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::LEDGE_INTANGIBILITY);
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
    }

    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::JUMP_FROM_SQUAT) {
        if VarModule::get_int(fighter.module_accessor, vars::fighter::instance::int::JUMP_FROM_SQUAT_COUNT_STATUS) < 2
        && ![
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_FLY,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
        ].contains(&status)
       && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            VarModule::inc_int(fighter.module_accessor, vars::fighter::instance::int::JUMP_FROM_SQUAT_COUNT_STATUS);
        }
        else {
            VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::JUMP_FROM_SQUAT);
            VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::SUPER_JUMP);
            VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::SUPER_JUMP_SET_MOMENTUM);
            VarModule::set_int(fighter.module_accessor, vars::fighter::instance::int::JUMP_FROM_SQUAT_COUNT_STATUS, 0);
        }
    }

    // set the combo counter to red
    // brother why am I red
    let hit_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_HIT_COUNT);
    if hit_count != 0 {
        if [
            *FIGHTER_STATUS_KIND_DOWN,
            *FIGHTER_STATUS_KIND_DOWN_CONTINUE,
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
            *FIGHTER_STATUS_KIND_SLIP_WAIT,
        ].contains(&status) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUCCEED_HIT_COUNT_RED);
        }
        else if [
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
        ].contains(&status) {
            let can_tech = fighter.is_enable_passive().get_bool();
            let passives = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
            ];
            let mut tech_transition_check = false;
            for transition in passives.iter() {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *transition) {
                    tech_transition_check = true;
                    break;
                }
            }
    
            if can_tech && tech_transition_check {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUCCEED_HIT_COUNT_RED);
            }
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_fighter_pre_end_status
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}