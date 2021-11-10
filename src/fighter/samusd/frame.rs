use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CFighterBase},
        hash40,
        phx::Vector3f,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

#[inline(always)]
pub unsafe fn samusd_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    let mut allowed_cancels : Vec<i32> = [].to_vec();
    set_hp(fighter, 110.0);
    if [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].contains(&status) {
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status) {
        allowed_cancels = [
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI
        ].to_vec();
    }
    else if [
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&status)
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b")
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
        jump_cancel_check_hit(fighter, false);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_S4 {
        dash_cancel_check(fighter, false, false);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
        dash_cancel_check(fighter, false, true);
    }
    cancel_system(fighter, status, allowed_cancels);
}

#[fighter_frame( agent = FIGHTER_KIND_SAMUSD )]
fn samusd_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            BOUNCE[entry_id(fighter.module_accessor)] = false;
        }
        if sv_information::is_ready_go() == false {
            BOUNCE[entry_id(fighter.module_accessor)] = false;
        }
    
        // Morph Ball Drop Bounce
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw") {
            if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && BOUNCE[entry_id(fighter.module_accessor)] == false {
                MotionModule::set_frame_sync_anim_cmd(
                    fighter.module_accessor,
                    44.0,
                    true,
                    true,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0,y: 0.5,z: 0.0});
                BOUNCE[entry_id(fighter.module_accessor)] = true;
            }
        }
        else {
            if BOUNCE[entry_id(fighter.module_accessor)] {
                KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0,y: 0.25,z: 0.0});
                BOUNCE[entry_id(fighter.module_accessor)] = false;
            }
        }

        if IS_FGC[entry_id(fighter.module_accessor)] {
            samusd_fgc(fighter);
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_SAMUSD_CSHOT )]
fn samusd_cshot_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if MotionModule::motion_kind(weapon.module_accessor) == smash::hash40("shoot") {
            let slowdownvec : Vector3f = Vector3f{x: 0.9,y: 0.0,z: 0.0};
            KineticModule::mul_speed(weapon.module_accessor, &slowdownvec, *WEAPON_KINETIC_TYPE_NONE);
        }
    }
}

// #[status_script(agent = "samusd_missile", status = WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn samusd_missile_homing_main(weapon: &mut L2CFighterBase) -> L2CValue {
//     MotionModule::change_motion(weapon.module_accessor, Hash40::new("homing"), 0.0, 1.0, false, 0.0, false, false);
//     if !StopModule::is_stop(weapon.module_accessor) {
//         WorkModule::dec_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE);
//     }
//     weapon.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(samusd_missile_dec_int as *const () as _));
//     weapon.fastshift(L2CValue::Ptr(samusd_missile_homing_main_loop as *const () as _))
// }

// unsafe extern "C" fn samusd_missile_dec_int(weapon: &mut L2CFighterBase) -> L2CValue {
//     WorkModule::dec_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE);
//     0.into()
// }

// unsafe extern "C" fn samusd_missile_homing_main_loop(weapon: &mut L2CFighterBase) -> L2CValue {
//     if !StatusModule::is_changing(weapon.module_accessor) {
//         if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
//             if !WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
//                 return 0.into();
//             }
//         }
//         println!("burstin' time");
//         weapon.change_status(WEAPON_SAMUS_MISSILE_STATUS_KIND_H_BURST.into(), false.into());
//     }
//     0.into()
// }

pub fn install() {
    install_agent_frames!(
        samusd_frame,
        samusd_cshot_frame
    );
}