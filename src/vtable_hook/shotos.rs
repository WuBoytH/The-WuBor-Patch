use crate::imports::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::shotos::WHAT_IS_THIS)]
extern "C" fn what_is_this(fighter: &mut Fighter) -> *const u64;

#[skyline::from_offset(offsets::fighter::shotos::WHAT_IS_THIS_2)]
extern "C" fn what_is_this_2(param_1: i32, param_2: i32, param_3: u64, module_accessor: *mut BattleObjectModuleAccessor, huh: *const u64);

#[skyline::hook(offset = offsets::fighter::shotos::HANDLE_LIGHT_NORMALS)]
unsafe extern "C" fn ryu_ken_handle_light_normals(fighter: &mut Fighter, heavy_motion: u64, light_motion: u64) {
    let module_accessor = fighter.battle_object.module_accessor;
    let mot = MotionModule::motion_kind(module_accessor);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (!StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor))
    && (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 2) {
        if !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK) {
            if (mot ^ heavy_motion) & 0xffffffffff == 0 {
                let button = ControlModule::get_button(module_accessor);
                let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
                if button & 1 == 0
                && (cat1 >> 0x15 & 1 == 0 || !WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON)) {
                    HitModule::set_status_all(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
                    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
                    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
                    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
                    WorkModule::off_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
                    // MotionModule::change_motion_inherit_frame(
                    //     module_accessor,
                    //     Hash40::new_raw(light_motion),
                    //     -1.0,
                    //     1.0,
                    //     0.0,
                    //     false,
                    //     false
                    // );
                    MotionModule::change_motion(
                        module_accessor,
                        Hash40::new_raw(light_motion),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                    WorkModule::on_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
                    WorkModule::on_flag(module_accessor, 0x21000022 + 6);
                }
            }
        }
        else if (mot ^ light_motion) & 0xffffffffff == 0
        && !WorkModule::is_flag(module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG)
        && WorkModule::get_int(module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) < 2 {
            let something = match mot {
                0xb4823ab96 => 0x15, // attack_11_w
                0xc226d08d1 => 0x17, // attack_hi3_w
                0xc672b8ebd => 0x18, // attack_lw3_w
                0xdebddf8ff => 0x16, // attack_s3_s_w
                0xdf2f03674 => 0x1a, // attack_near_w
                _ => 1
            };
            let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
            if (cat1 >> 0x15 & 1 == 0 || !WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON))
            && ControlModule::get_command_flag_cat(module_accessor, 3) == 0 {

                let huh = what_is_this(fighter);

                what_is_this_2(2, something, 1, module_accessor, huh);
            }
            WorkModule::on_flag(module_accessor, 0x21000022 + 7);
        }
    }
}

pub fn install() {
    unsafe {
        // Patches the original final smash cancel check
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shotos::FINAL_SMASH_CANCEL_CHECK_PATCH).data(0x14000039u32);

        // Allows any status over 0x1de to be final smash cancelable
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shotos::FINAL_SMASH_CANCEL_STATUS_PATCH).data(0x1400000Au32);

        // Some Kind of Transition Check
        // Ryu
        let _ = skyline::patching::Patch::in_text(offsets::fighter::shotos::RYU_TRANSITION_CHECK_TABLE + 0x1F8).data(1u8);

        skyline::install_hooks!(
            ryu_ken_handle_light_normals
        );
    }
}
