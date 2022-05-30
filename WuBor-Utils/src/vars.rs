#![allow(non_upper_case_globals)]

use {
    std::sync::Once,
    smash::{
        phx::Vector3f,
        app::*,
    }
};

// System
pub static mut INT_OFFSET : usize = 0x4E19D0;
// pub static mut INT64_OFFSET : usize = 0x4E19F0;
pub static mut FLOAT_OFFSET : usize = 0x4E19D0;
pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
pub static mut DEFINE_LUA_CONSTANT_OFFSET : usize = 0x3727390; //13.0.1
pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];
pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];
// pub static INT64_SEARCH_CODE: &[u8] = &[
//     0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x15, 0x40, 0xf9,
// ];
pub static NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1,
    0xe8, 0x2b, 0x00, 0xfd,
    0xfc, 0x6f, 0x06, 0xa9,
    0xfa, 0x67, 0x07, 0xa9,
    0xf8, 0x5f, 0x08, 0xa9,
    0xf6, 0x57, 0x09, 0xa9,
    0xf4, 0x4f, 0x0a, 0xa9,
    0xfd, 0x7b, 0x0b, 0xa9,
    0xfd, 0xc3, 0x02, 0x91,
    0xfb, 0x03, 0x00, 0xaa
];

// Common
pub const ZERO_VECTOR : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

// System Mechanics
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_GUARD_OFF_ATTACK_CANCEL : i32 = 0x20000116;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N : i32 = 0x20000117;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S : i32 = 0x20000118;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI : i32 = 0x20000119;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW : i32 = 0x2000011A;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF : i32 = 0x2000011B;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC : i32 = 0x2000011C;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB : i32 = 0x2000011D;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_SUPER_JUMP : i32 = 0x2000011E;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_ESCAPE_AIR_SLIDE : i32 = 0x2000011F;
pub static mut FGC_TRAINING : bool = false;

pub const FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID : i32 = 0x100000ED;
pub const FIGHTER_INSTANCE_WORK_ID_INT_USED_GROUND_NORMALS : i32 = 0x100000EE;
pub const FIGHTER_INSTANCE_WORK_ID_INT_USED_AERIALS : i32 = 0x100000EF;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP : i32 = 0x100000F0;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_TIMER : i32 = 0x100000F1;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP : i32 = 0x100000F2;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_TIMER : i32 = 0x100000F3;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP : i32 = 0x100000F4;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_TIMER : i32 = 0x100000F5;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_STEP : i32 = 0x100000F6;
// pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236236_TIMER : i32 = 0x100000F7;

pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_PREV : i32 = 0x5F;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_FLICK_DOWN : i32 = 0x60;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_SUPER_JUMP_FRAME : i32 = 0x61;

pub const FIGHTER_STATUS_GUARD_OFF_WORK_INT_ATTACK_CANCEL_FRAME : i32 = 0x1100000F;

pub const FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD : i32 = 0x2100000D;
pub const FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_LOOP : i32 = 0x2100000E;
pub const FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_ENABLE_ACTION : i32 = 0x2100000F;

pub const FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_LOOP_MOT : i32 = 0x11000007;
pub const FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON : i32 = 0x11000008;
pub const FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME : i32 = 0x11000009;
pub const FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_ACTION_MOT : i32 = 0x1100000A;
pub const FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_ACTION_BUTTON : i32 = 0x1100000B;

// pub const FIGHTER_STATUS_ESCAPE_AIR_FLAG_FORCE_SLIDE_F : i32 = 21000019;

pub const FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER : i32 = 0x1000026;

pub const FIGHTER_STATUS_WORK_ID_INT_ENABLED_AERIALS : i32 = 0x11000026;
// Set int like so: WorkModule::set_int(fighter.module_accessor, 0b11111, FIGHTER_STATUS_WORK_ID_INT_ENABLED_AERIALS);
// Bitmask is 0bXXXXX, where each X represents an aerial. 0b(LW)(HI)(B)(F)(N)

pub const ATTACK_AIR_N_MASK : i32 = 0b00001;
pub const ATTACK_AIR_F_MASK : i32 = 0b00010;
pub const ATTACK_AIR_B_MASK : i32 = 0b00100;
pub const ATTACK_AIR_HI_MASK : i32 = 0b01000;
pub const ATTACK_AIR_LW_MASK : i32 = 0b10000;

pub const ATTACK_N_MASK : i32 = 0b0000001;
pub const ATTACK_S3_MASK : i32 = 0b0000010;
pub const ATTACK_HI3_MASK : i32 = 0b0000100;
pub const ATTACK_LW3_MASK : i32 = 0b0001000;
pub const ATTACK_S4_MASK : i32 = 0b0010000;
pub const ATTACK_HI4_MASK : i32 = 0b0100000;
pub const ATTACK_LW4_MASK : i32 = 0b1000000;

pub const FIGHTER_STATUS_WORK_ID_FLAG_JUMP_CANCEL : i32 = 0x2100002B;
// pub const FIGHTER_STATUS_WORK_ID_FLAG_DASH_CANCEL : i32 = 0x2100002C;
pub const FIGHTER_STATUS_WORK_ID_FLAG_NORMAL_CANCEL : i32 = 0x2100002D;
// pub const FIGHTER_STATUS_WORK_ID_FLAG_SPECIAL_CANCEL : i32 = 0x2100002E;

pub mod singletons {
    // All credit for this to blujay, macros are very cool
    use super::*;
    use skyline::nn::ro::LookupSymbol;
    
    static INIT : Once = Once::new();

    pub static mut FIGHTER_MANAGER : *const *mut smash::app::FighterManager = 0 as _;
    pub static mut FIGHTER_CUTIN_MANAGER : *const *mut smash::app::FighterCutInManager = 0 as _;

    macro_rules! expose_singleton {
        ($($public:ident, $private:ident)*) => {
            $(
                #[inline(always)]
                #[allow(non_snake_case)]
                pub fn $public() -> *mut $public {
                    unsafe {
                        *$private
                    }
                }
            )*
        }
    }

    macro_rules! assign_symbol {
        ($id:ident, $e:expr) => {{
            unsafe {
                let mut sym = 0usize;
                LookupSymbol(&mut sym as *mut usize, $e.as_ptr() as _);
                assert!(sym != 0, "Failed to find symbol {}", $e);
                $id = std::mem::transmute(sym)
            }
        }}
    }

    expose_singleton!(
        FighterManager, FIGHTER_MANAGER
        FighterCutInManager, FIGHTER_CUTIN_MANAGER
    );

    pub fn init() {
        INIT.call_once(|| {
            assign_symbol!(
                FIGHTER_MANAGER,
                "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\0"
            );
            assign_symbol!(
                FIGHTER_CUTIN_MANAGER,
                "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\0"
            );
        });
    }
}

pub fn install() {
    singletons::init();
}
