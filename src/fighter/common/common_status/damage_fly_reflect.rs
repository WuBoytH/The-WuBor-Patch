// #![allow(improper_ctypes)]
use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::table_const::*
};

// extern "C" {
//     #[link_name = "\u{1}_ZN3app8lua_bind32GroundModule__get_touch_pos_implEPNS_26BattleObjectModuleAccessorEj"]
//     pub fn get_touch_pos_simd(
//         module_accessor: *mut smash::app::BattleObjectModuleAccessor,
//         arg2: libc::c_uint,
//     ) -> smash_rs::cpp::simd::Vector2;
// }

// #[skyline::hook(replace = L2CFighterCommon_sub_DamageFlyReflect_effect)]
// unsafe fn sub_damageflyreflect_effect(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
//     let touch_flag = GroundModule::get_touch_flag(fighter.module_accessor);
//     if touch_flag == 0 {
//         return;
//     }
//     let mut pos = smash_rs::cpp::simd::Vector2{x: 0.0, y: 0.0};
//     let mut normal = Vector2f{x: 0.0, y: 0.0};
//     let flags = [
//         *GROUND_TOUCH_FLAG_DOWN,
//         *GROUND_TOUCH_FLAG_UP,
//         *GROUND_TOUCH_FLAG_LEFT,
//         *GROUND_TOUCH_FLAG_RIGHT
//     ];
//     for x in flags.iter() {
//         if touch_flag & *x as u64 != 0 {
//             pos = get_touch_pos_simd(fighter.module_accessor, *x as u32);
//             normal = Vector2f{
//                 x: GroundModule::get_touch_normal_x(fighter.module_accessor, *x as u32),
//                 y: GroundModule::get_touch_normal_y(fighter.module_accessor, *x as u32)
//             };
//             break;
//         }
//     }
//     println!("Touch Pos: x: {}, y: {}", pos.x, pos.y);
//     let atan = (-normal.x).atan2(normal.y);
//     let mut effect = hash40("sys_crown");
//     if touch_flag & *GROUND_TOUCH_FLAG_DOWN as u64 == 0
//     && touch_flag & *GROUND_TOUCH_FLAG_UP as u64 == 0 {
//         let stage = smash_rs::app::sv_information::stage_id();
//         if stage == 0x152 { // Dolly_Stadium
//             fighter.clear_lua_stack();
//             lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
//             let damage_speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
//             let dolly_stage_disable_wall_crash_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dolly_stage_disable_wall_crash_speed_x"));
//             let mut se = 0x249c66306cu64;
//             if dolly_stage_disable_wall_crash_speed_x <= damage_speed_x {
//                 let break_speed = fighter.FighterStatusDamage__get_dolly_stadium_wall_break_speed().get_f32();
//                 let crash_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dolly_stage_wall_crash_speed_x"));
//                 let effect_min_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dolly_stage_wall_effect_min_rate"));
//                 let effect_middle_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dolly_stage_wall_effect_middle_rate"));
//                 let min_ratio = effect_min_rate / 100.0;
//                 let mid_ratio = effect_middle_rate / 100.0;
//                 if 0.0 < break_speed {
//                     if break_speed < crash_speed_x * min_ratio {
//                         effect = 0x195581fdb4u64;
//                         se = 0x2466690d0fu64;
//                     }
//                     else if break_speed < crash_speed_x * mid_ratio {
//                         effect = 0x19cc88ac0eu64;
//                         se = 0x249c66306cu64;
//                     }
//                     else if !smash_rs::app::FighterCutInManager::is_one_on_one() {
//                         effect = 0x192286cd22u64;
//                         se = 0x24116e3d99u64;
//                     }
//                     else {
//                         let slow_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dolly_stage_wall_effect_max_slow_rate"));
//                         let slow_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dolly_stage_wall_effect_max_slow_frame"));
//                         SlowModule::set_whole(fighter.module_accessor, slow_rate as u8, slow_frame);
//                     }
//                 }
//             }
//             else {
//                 se = 0x249c66306cu64;
//             }
//             SoundModule::play_se(
//                 fighter.module_accessor,
//                 Hash40::new_raw(se),
//                 true,
//                 false,
//                 false,
//                 false,
//                 enSEType(0)
//             );
//         }
//     }
//     let pos_z = GroundModule::get_z(fighter.module_accessor);
//     EffectModule::req(
//         fighter.module_accessor,
//         Hash40::new_raw(effect),
//         &Vector3f{x: pos.x, y: pos.y, z: pos_z},
//         &Vector3f{x: 0.0, y: 0.0, z: atan},
//         1.0,
//         *EFFECT_SUB_ATTRIBUTE_NONE as u32,
//         -1,
//         false,
//         0
//     );
    
//     // Vanilla

//     // if fighter.is_enable_passive().get_bool()
//     // && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
//     //     // Nothing lmao
//     // }
//     // else {
//     //     // play untechable effect
//     // }

//     // New
//     // Because you're only untechable if you're in the Damage Fly Roll state or if that disable passive flag is on,
//     // I want to change this behavior to play this effect only if the previous status was Damage Fly Roll/if the flag is on.
//     if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
//     || fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
//     || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
//         let pos_z = GroundModule::get_z(fighter.module_accessor);
//         EffectModule::req(
//             fighter.module_accessor,
//             Hash40::new_raw(0xde89fce0a),
//             &Vector3f{x: pos.x, y: pos.y, z: pos_z},
//             &Vector3f{x: 0.0, y: 0.0, z: atan},
//             1.0,
//             *EFFECT_SUB_ATTRIBUTE_NONE as u32,
//             -1,
//             false,
//             0
//         );
//     }
//     if param_1.get_bool() {
//         CameraModule::req_quake_pos(fighter.module_accessor, *CAMERA_QUAKE_KIND_SMALL, &Vector3f{x: pos.x, y: pos.y, z: 0.0});
//     }
// }

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyReflectD)]
unsafe fn status_damageflyreflectd(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FLY_REFLECT_COUNT);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    let passive = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in passive.iter() {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_MIIFIGHTER_100KICK_SET_REFLECT_ANGLE) {
        let mut some = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x25e72e042c);
        let miifighter_100kick_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_MIIFIGHTER_100KICK_LR);
        if miifighter_100kick_lr < 0.0 {
            some = 180.0 - some;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed_length = sv_kinetic_energy::get_speed_length(fighter.lua_state_agent);
        let speed_x = speed_length * some.to_radians().cos();
        let speed_y = speed_length * some.to_radians().sin();
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_DAMAGE,
            speed_x,
            speed_y
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_DamageFlyReflectD_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyReflectLR)]
unsafe fn status_damageflyreflectlr(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FLY_REFLECT_COUNT);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    let passive = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in passive.iter() {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    }
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
    SoundModule::play_se(
        fighter.module_accessor,
        Hash40::new_raw(0x157ce9b678),
        true,
        false,
        false,
        false,
        enSEType(0)
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_DamageFlyReflectLR_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyReflectU)]
unsafe fn status_damageflyreflectu(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FLY_REFLECT_COUNT);
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    let passive = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE
    ];
    for x in passive.iter() {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE) {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        else {
            WorkModule::enable_transition_term(fighter.module_accessor, *x);
        }
    }
    fighter.sub_DamageFlyCommon_init();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.damage_air_chk_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_damage_fly_chk_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_DamageFlyReflectU_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            // sub_damageflyreflect_effect,
            status_damageflyreflectd,
            status_damageflyreflectlr,
            status_damageflyreflectu
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}