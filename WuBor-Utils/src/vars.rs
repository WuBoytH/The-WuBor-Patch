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
pub static mut FGC_TRAINING : bool = false;

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

pub mod commons {
    pub mod instance {
        pub mod flag {
            pub const DISABLE_SPECIAL_N : i32 = 0x0000;
            pub const DISABLE_SPECIAL_S : i32 = 0x0001;
            pub const DISABLE_SPECIAL_HI : i32 = 0x0002;
            pub const DISABLE_SPECIAL_LW : i32 = 0x0003;
            pub const GUARD_OFF_ATTACK_CANCEL : i32 = 0x0004;
            pub const IS_FGC : i32 = 0x0005;
            pub const CANCEL_ESCAPE_TO_ESCAPE_FB : i32 = 0x0006;
            pub const SUPER_JUMP : i32 = 0x0007;
            pub const FORCE_ESCAPE_AIR_SLIDE : i32 = 0x0008;
        }
        pub mod int {
            pub const TARGET_ID : i32 = 0x0000;
            pub const USED_GROUND_NORMALS : i32 = 0x0001;
            pub const USED_AERIALS : i32 = 0x0002;
            // pub const CUSTOM_COMMAND_236_STEP : i32 = 0x0003;
            // pub const CUSTOM_COMMAND_236_TIMER : i32 = 0x0004;
            // pub const CUSTOM_COMMAND_214_STEP : i32 = 0x0005;
            // pub const CUSTOM_COMMAND_214_TIMER : i32 = 0x0006;
            // pub const CUSTOM_COMMAND_623_STEP : i32 = 0x0007;
            // pub const CUSTOM_COMMAND_623_TIMER : i32 = 0x0008;
            // pub const CUSTOM_COMMAND_236236_STEP : i32 = 0x0009;
            // pub const CUSTOM_COMMAND_236236_TIMER : i32 = 0x000A;
        }
        pub mod float {
            pub const FLICK_DOWN : i32 = 0x0000;
            pub const SUPER_JUMP_FRAME : i32 = 0x0001;
        }
    }
    pub mod status {
        pub mod flag {
            pub const JUMP_CANCEL : i32 = 0x1000;
            pub const NORMAL_CANCEL : i32 = 0x1001;
            // pub const DASH_CANCEL : i32 = 0x1002;
            // pub const SPECIAL_CANCEL : i32 = 0x1003;
        }
        pub mod int {
            pub const ENABLED_AERIALS : i32 = 0x1000;
        }
        pub mod float {
            pub const HIT_FRAME : i32 = 0x1000;
        }
    }
}

pub mod attack_air {
    pub mod flag {
        pub const WHIFF : i32 = 0x1000;
    }
}

pub mod guard_off {
    pub mod int {
        pub const ATTACK_CANCEL_FRAME : i32 = 0x1000;
    }
}

pub mod appeal {
    pub mod flag {
        pub const HOLD : i32 = 0x1000;
        pub const LOOP : i32 = 0x1001;
        pub const ENABLE_ACTION : i32 = 0x1002;
    }
    pub mod int {
        pub const HOLD_BUTTON : i32 = 0x1000;
        pub const ACTION_BUTTON : i32 = 0x1001;
        pub const RESTART_FRAME : i32 = 0x1002;
    }
    pub mod int64 {
        pub const ACTION_MOT : i32 = 0x1000;
        pub const LOOP_MOT : i32 = 0x1001;
    }
}

pub mod bayonetta {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_AIR_S_D_IS_BOUNCE : i32 = 0x1100;
        }
    }
}

pub mod chrom {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_LW_CHANGE_KINETIC : i32 = 0x1100;
        }
    }
}

pub mod dolly {
    pub mod instance {
        pub mod flag {
            pub const IS_SPECIAL_CANCEL : i32 = 0x0100;
            pub const ATTACK_DASH_COMMAND : i32 = 0x0101;
            pub const RISING_FORCE : i32 = 0x0102;
        }
        pub mod int {
            pub const D_TILT_CHAIN_COUNT : i32 = 0x0100;
            pub const SUPER_SPECIAL_AURA : i32 = 0x0101;
            pub const SUPER_SPECIAL_AURA2 : i32 = 0x0102;
        }
        pub mod float {
            pub const GO_METER : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_N_FEINT : i32 = 0x1100;
        }
    }
}

pub mod dolly_wave {
    pub mod instance {
        pub mod flag {
            pub const FROM_CANCEL : i32 = 0x0100;
        }
    }
}

pub mod donkey {
    pub static mut DK_COUNT : u64 = 0;
}

pub mod edge {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_HI_CANCEL : i32 = 0x1100;
        }
        pub mod int {
            pub const SPECIAL_HI_CANCEL_COUNT : i32 = 0x0100;
        }
    }
}

pub mod eflame {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_S_ROTATE : i32 = 0x1100;
        }
    }
}

pub mod elight {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_S_CANCEL : i32 = 0x0100;
        }
    }
}

pub mod falco {
    pub mod instance {
        pub mod flag {
            pub const KAA : i32 = 0x0100;
        }
    }
}

pub mod ganon {
    pub mod status {
        pub mod flag {
            pub const TELEPORT_FEINT : i32 = 0x1100;
            pub const TELEPORT_STOP : i32 = 0x1101;
            pub const TELEPORT_START_GROUND : i32 = 0x1102;
        }
        pub mod int {
            pub const TELEPORT_STEP : i32 = 0x1100;
        }
        pub mod float {
            pub const START_POS: i32 = 0x1100;

            pub const END_POS: i32 = 0x1102;
        }
    }

    pub const TELEPORT_STEP_INIT : i32 = 1;
    pub const TELEPORT_STEP_MOVE : i32 = 2;
    pub const TELEPORT_STEP_MOVE_DONE : i32 = 3;
    pub const TELEPORT_STEP_CHECK_FEINT : i32 = 4;
    pub const TELEPORT_STEP_END : i32 = 5;
}

pub mod gaogaen {
    pub mod instance {
        pub mod int {
            pub const REVENGE : i32 = 0x0100;
        }
    }
}

pub mod jack {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_S_FEINT : i32 = 0x1100;
        }
    }
}

pub mod kamui {
    pub mod instance {
        pub mod float {
            pub const DRAGON_INSTALL : i32 = 0x0100;
            pub const DRAGON_INSTALL_TIMER : i32 = 0x0101;
        }
    }
}

pub mod ken {
    pub mod instance {
        pub mod flag {
            pub const V_TRIGGER : i32 = 0x0100;
        }
        pub mod int {
            pub const SPECIAL_LW_TYPE : i32 = 0x0100;
            pub const QUICK_STEP_STATE : i32 = 0x0101;
            pub const FLASH_MAX : i32 = 0x0102;
            pub const FLASH_COUNTER : i32 = 0x0103;
            pub const SHORYUREPPA : i32 = 0x0104;
            pub const V_TRIGGER_EFF_TIMER : i32 = 0x0105;
        }
        pub mod float {
            pub const V_GAUGE : i32 =  0x0100;
            pub const DIFF_X : i32 = 0x0101;
        }
    }
    pub mod status {
        pub mod flag {
            pub const VS1_CANCEL : i32 = 0x1100;
            pub const VT1_CANCEL : i32 = 0x1101;
            pub const GUARD_V_SHIFT : i32 = 0x1102;
            pub const SPECIAL_LW_STEP_KICK : i32 = 0x1102;
        }
    }

    pub const SPECIAL_LW_TYPE_QUICK_STEP : i32 = 0;
    pub const SPECIAL_LW_TYPE_HEAT_RUSH : i32 = 1;

    pub const QUICK_STEP_STATE_ENABLE : i32 = 0;
    pub const QUICK_STEP_STATE_RUN : i32 = 1;
    pub const QUICK_STEP_STATE_DISABLE : i32 = 2;
}

pub mod kirby {
    pub mod status {
        pub mod flag {
            pub const ATTACK_DASH_END : i32 = 0x1100;
            pub const ATTACK_LW3_BOUNCE : i32 = 0x1100;
        }
        pub mod int {
            pub const APPEAL_S_LOOP_COUNT : i32 = 0x1100;
        }
    }
}

pub mod lucario {
    pub mod instance {
        pub mod flag {
            pub const IS_SUPER_DASH_CANCEL : i32 = 0x0100;
        }
        pub mod int {
            pub const AURA_LEVEL : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const IS_AURA_ENHANCED : i32 = 0x1100;

            pub const SPECIAL_LW_ENABLE_CANCEL : i32 = 0x1150;
        }
    }
}

pub mod yu { // lucina
    use super::*;

    pub mod instance {
        pub mod flag {
            pub const DISABLE_SPECIAL_N_S : i32 = 0x0100;
            pub const AWAKENING : i32 = 0x0101;
            pub const SHADOW_FRENZY : i32 = 0x0102;
            pub const ROMAN_ON_HIT : i32 = 0x0103;
            pub const HEROIC_GRAB : i32 = 0x0104;
            pub const COMMAND : i32 = 0x0105;
        }
        pub mod int {
            pub const SP_LEVEL : i32 = 0x0100;
            pub const SP_EFFECT_TIMER : i32 = 0x0101;
            pub const SP_FLASH_TIMER : i32 = 0x0102;
            pub const SP_GLOW_TIMER : i32 = 0x0103;
            pub const SHADOW_EFF_ID : i32 = 0x0104;
        }
        pub mod float {
            pub const SP_GAUGE : i32 = 0x0100;
            pub const SPENT_SP : i32 = 0x0101;
            pub const SP_GAUGE_MAX : i32 = 0x0102;
            pub const SP_GAIN_PENALTY : i32 = 0x0103;
        }
    }
    pub mod status {
        pub mod flag {
            pub const IS_EX : i32 = 0x1100;
            pub const CAN_ONE_MORE : i32 = 0x1101;
            pub const ATTACK_DASH_BIG_GAMBLE : i32 = 0x1102;
            pub const SPECIAL_LW_DECIDE_ROMAN_DIREC : i32 = 0x1103;
            pub const SPECIAL_LW_ROMAN_MOVE : i32 = 0x1104;
        }
        pub mod float {
            pub const SPECIAL_LW_ROMAN_MOVE : i32 = 0x1100;
        }
    }

    pub const SP_1 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -6.0};
    pub const SP_2 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -2.0};
    pub const SP_3 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 2.0};
    pub const SP_4 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 6.0};
    pub const SP_5 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: -2.0};
    pub const SP_6 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: 2.0};

    pub static YU_AUDIO: [&'static str; 36] = ["appeal01", "appeal02", "attack01", "attack02", "attack03", "attack04", "attack05", "attack06", "attack07", "cliffcatch", "damage_twinkle", "damage01", "damage02", "damage03", "damagefly01", "damagefly02", "final", "furafura", "furasleep", "heavyget", "jump01", "missfoot01", "missfoot02", "ottotto", "passive", "special_h01", "special_l01", "special_l02", "special_n01", "swimup", "win01", "win02", "win03", "win_marth", "win_ike", "knockout"];
    pub static YU_SEQ: [&'static str; 8] = ["attack", "special_n", "special_l", "special_h", "futtobi01", "futtobi02", "jump", "ottotto"];
}

pub mod luigi {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_HI_CANCEL : i32 = 0x0100;
        }
    }
}

pub mod mario {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_LW_BLJ_PREV : i32 = 0x0100;
        }
        pub mod int {
            pub const SPECIAL_LW_KIND : i32 = 0x0100;
        }
    }
    pub mod special_n {
        pub mod flag {
            pub const FGC_CANCEL : i32 = 0x1100;
        }
    }
    pub mod special_lw {
        pub mod flag {
            pub const LANDING : i32 = 0x1100;
            pub const BLJ : i32 = 0x1101;
        }
        pub mod int {
            pub const LONG_JUMP_KIND : i32 = 0x1100;
        }
    }
    pub const SPECIAL_LW_KIND_LONG_JUMP : i32 = 0;
    pub const SPECIAL_LW_KIND_GROUND_POUND : i32 = 1;
    pub const SPECIAL_LW_KIND_GROUND_POUND_CANCEL : i32 = 2;

    pub const LONG_JUMP_W : i32 = 0;
    pub const LONG_JUMP_M : i32 = 1;
    pub const LONG_JUMP_S : i32 = 2;
    pub const LONG_JUMP_B : i32 = 3;
}

pub mod marth {
    pub mod instance {
        pub mod flag {
            pub const IS_STANCE : i32 = 0x0100;
            pub const PARRY_XLU : i32 = 0x0101;
            pub const AIR_STANCE : i32 = 0x0102;
        }
        pub mod int {
            pub const STANCE_CHANGE_LOCKOUT : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const DISABLE_STANCE_CHANGE : i32 = 0x1100;
            pub const ATTACK_3_CHANGE_MOTION : i32 = 0x1101;
            pub const ATTACK_F3_HEAVY : i32 = 0x1102;
            pub const SPECIAL_S_FLASHING_BLADE : i32 = 0x1101;
            pub const SPECIAL_S_DASH : i32 = 0x1102;
            pub const SPECIAL_S_END : i32 = 0x1103;
            pub const SPECIAL_S2_FINAL_BLOW : i32 = 0x1104;
        }
        pub mod int {
            pub const SPECIAL_S2_START_SITUATION : i32 = 0x1100;
            pub const SPECIAL_S2_LOOP_COUNT : i32 = 0x1101;
        }
        pub mod float {
            pub const SPECIAL_S_ANGLE : i32 = 0x1100;
        }

        pub const STANCE_ENTER : i32 = 0;
        pub const STANCE_WAIT : i32 = 1;
        pub const STANCE_SQUAT : i32 = 2;
        pub const STANCE_SQUAT_WAIT : i32 = 3;
        pub const STANCE_SQUAT_RV : i32 = 4;
        pub const STANCE_EXIT : i32 = 5;
        pub const STANCE_DASH_F : i32 = 6;
        pub const STANCE_DASH_B : i32 = 7;
        pub const STANCE_ATTACK : i32 = 8;
        pub const STANCE_ATTACK_LW3 : i32 = 9;
        pub const STANCE_ATTACK_LW4 : i32 = 10;
        pub const STANCE_ATTACK_HI3 : i32 = 11;
        pub const STANCE_ATTACK_B3 : i32 = 12;
        pub const STANCE_ATTACK_F3 : i32 = 13;
        pub const STANCE_SPECIAL_S : i32 = 14;
        pub const STANCE_SPECIAL_S_DASH : i32 = 15;
        pub const STANCE_SPECIAL_S_END : i32 = 16;
        pub const STANCE_SPECIAL_S2_START : i32 = 17;
        pub const STANCE_SPECIAL_S2_LOOP : i32 = 18;
        pub const STANCE_SPECIAL_S2_END : i32 = 19;
        pub const STANCE_SPECIAL_S2_END2 : i32 = 20;
        // pub const STANCE_SPECIAL_S2_LANDING : i32 = 21;
    }
}

pub mod ryu {
    pub mod instance {
        pub mod flag {
            pub const DISABLE_EX_FOCUS : i32 = 0x0100;
            pub const EX_FOCUS : i32 = 0x0101;
            pub const EX_FLASH : i32 = 0x0102;
            pub const SEC_SEN_STATE : i32 = 0x0103;
            pub const SECRET_SENSATION : i32 = 0x0104;
            pub const SEC_SEN_CAMERA : i32 = 0x0105;
        }
        pub mod int {
            pub const FLASH_TIMER : i32 = 0x0100;
        }
        pub mod float {
            pub const RYU_X : i32 = 0x0100;
            pub const RYU_Y : i32 = 0x0101;
            pub const TARGET_X : i32 = 0x0102;
            pub const TARGET_Y : i32 = 0x0103;
            pub const DISABLE_EX_FOCUS_TIMER : i32 = 0x0104;
            pub const SEC_SEN_TIMER : i32 = 0x0105;
            pub const OPPONENT_DIREC : i32 = 0x0106;
            pub const VERT_EXTRA : i32 = 0x0107;
        }
    }
}

pub mod samusd {
    pub mod instance {
        pub mod flag {
            pub const ATTACK_AIR_N_FLOAT : i32 = 0x0100;
        }
        pub mod int {
            pub const CSHOT_ID : i32 = 0x0100;
        }
    }
    pub mod attack_air_n {
        pub mod flag {
            pub const START_FLOAT : i32 = 0x1100;
        }
    }
    pub mod special_lw {
        pub mod flag {
            pub const BOUNCE : i32 = 0x1100;
        }
    }
}

pub mod shizue {
    pub mod instance {
        pub mod flag {
            pub const FIRE_ROCKET_ANYTIME : i32 = 0x0100;
        }
    }
}

pub mod shulk {
    pub mod instance {
        pub mod float {
            pub const BURST_COOLDOWN : i32 = 0x0100;
        }
    }
}

pub mod szerosuit {
    pub mod special_hi {
        pub mod flag {
            pub const DECIDE_MOTION : i32 = 0x1100;
        }
    }
}

pub mod toonlink {
    pub mod attack_air_lw {
        pub mod flag {
            pub const BOUNCE : i32 = 0x1100;
        }
    }
    pub mod special_hi {
        pub mod float {
            pub const SPIN_SPEED : i32 = 0x1100;
            pub const SPIN_SPEED_MAX : i32 = 0x1101;
        }
    }
}

pub mod wario {
    pub mod throw {
        pub mod flag {
            pub const MOVE : i32 = 0x1100;
        }
        pub mod float {
            pub const SPIN_SPEED : i32 = 0x1100;
        }
    }
}

pub mod singletons {
    // All credit for this to blujay, macros are very cool
    use super::*;
    use skyline::nn::ro::LookupSymbol;
    
    static INIT : Once = Once::new();

    pub static mut FIGHTER_MANAGER : *const *mut smash::app::FighterManager = 0 as _;
    pub static mut FIGHTER_CUTIN_MANAGER : *const *mut smash::app::FighterCutInManager = 0 as _;

    extern "C" {
        #[link_name = "\u{1}_ZN3lib9SingletonIN3app16BattleObjectSlowEE9instance_E"]
        pub static mut BATTLE_OBJECT_SLOW: *mut u8;
    }
    
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
