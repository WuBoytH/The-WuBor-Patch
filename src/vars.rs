use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Vector3f,
        app::*,
    },
    smashline::*,
    crate::common_funcs::*
};

// System Vars
pub static mut INT_OFFSET : usize = 0x4E19D0;
// pub static mut INT64_OFFSET : usize = 0x4E19F0;
pub static mut FLOAT_OFFSET : usize = 0x4E19D0;
pub static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
// pub static mut MUSIC_OFFSET: usize = 0x3451f30; // default = 8.1.0 offset
// pub static mut MUSIC_PARAM1: *mut u64 = 0 as *mut u64;
// pub static mut MUSIC_PARAM2: i64 = 0;
// pub static mut NUS3AUDIO_HASH: *mut u64 = 0 as *mut u64;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut FIGHTER_MANAGER: usize = 0;
pub static mut ITEM_MANAGER: usize = 0;
pub static YU_AUDIO: [&'static str; 36] = ["appeal01", "appeal02", "attack01", "attack02", "attack03", "attack04", "attack05", "attack06", "attack07", "cliffcatch", "damage_twinkle", "damage01", "damage02", "damage03", "damagefly01", "damagefly02", "final", "furafura", "furasleep", "heavyget", "jump01", "missfoot01", "missfoot02", "ottotto", "passive", "special_h01", "special_l01", "special_l02", "special_n01", "swimup", "win01", "win02", "win03", "win_marth", "win_ike", "knockout"];
pub static YU_SEQ: [&'static str; 8] = ["attack", "special_n", "special_l", "special_h", "futtobi01", "futtobi02", "jump", "ottotto"];
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
// pub static MUSIC_SEARCH_CODE: &[u8] = &[
//     0xfc, 0x6f, 0xba, 0xa9, 0xfa, 0x67, 0x01, 0xa9, 0xf8, 0x5f, 0x02, 0xa9, 0xf6, 0x57, 0x03, 0xa9,
//     0xf4, 0x4f, 0x04, 0xa9, 0xfd, 0x7b, 0x05, 0xa9, 0xfd, 0x43, 0x01, 0x91, 0xff, 0xc3, 0x1b, 0xd1,
//     0xe8, 0x63, 0x05, 0x91,
// ];

// Common
pub const ZERO_VECTOR : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
pub static mut _TIME_COUNTER : [i32; 8] = [0; 8];

// System Mechanics
pub static mut COUNTER_HIT_STATE : [i32; 8] = [0; 8];
pub static mut IS_DK : [bool; 8] = [false; 8];
pub static mut CANCEL : [bool; 8] = [false; 8]; // Multi-purpose Cancel
pub static mut BOUNCE : [bool; 8] = [false; 8]; // Multi-purpose Bounce
pub static mut HIT_FRAME : [f32; 8] = [0.0; 8];
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N : i32 = 0x20000116;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S : i32 = 0x20000117;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI : i32 = 0x20000118;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_LW : i32 = 0x20000119;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF : i32 = 0x2000011A;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY : i32 = 0x2000011B;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC : i32 = 0x2000011C;
pub static mut FGC_TRAINING : bool = false;
pub static mut FGC_HITSTUN_MUL : [f32; 8] = [1.2; 8];
pub static mut SPECIAL_HITSTUN : [bool; 8] = [false; 8];
pub static mut HIT_BY_SPECIAL_HITSTUN : [bool; 8] = [false; 8];
pub static mut SPECIAL_LW_TYPE : [i32; 8] = [0; 8];
pub const FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME : i32 = 0x100000ED;
pub const FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID : i32 = 0x100000EE;
pub const FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER : i32 = 0x100000EF;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_236_STEP : i32 = 0x100000F0;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_214_STEP : i32 = 0x100000F1;
pub const FIGHTER_INSTANCE_WORK_ID_INT_CUSTOM_COMMAND_623_STEP : i32 = 0x100000F2;
pub const FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_PREV : i32 = 0x5F;

pub const FIGHTER_STATUS_WORK_ID_FLOAT_CANCEL_TIMER : i32 = 0x1000026;
pub const FIGHTER_STATUS_WORK_ID_FLAG_JUMP_CANCEL : i32 = 0x2100002B;
// pub const FIGHTER_STATUS_WORK_ID_FLAG_DASH_CANCEL : i32 = 0x2100002C;
// pub const FIGHTER_STATUS_WORK_ID_FLAG_NORMAL_CANCEL : i32 = 0x2100002D;
// pub const FIGHTER_STATUS_WORK_ID_FLAG_SPECIAL_CANCEL : i32 = 0x2100002E;

// Character Specific

// Mario
// pub static mut IS_BONKER : [i32; 8] = [0; 8];
pub const FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_FGC_CANCEL : i32 = 0x2100000E;
pub const FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_LANDING : i32 = 0x2100000B;
pub const FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_BLJ : i32 = 0x2100000C;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV : i32 = 0x200000E4;
pub const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND : i32 = 0x100000C0;
pub const FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP : i32 = 0;
pub const FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND : i32 = 1;
pub const FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND_CANCEL : i32 = 2;
pub const FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND : i32 = 0x11000005;
pub const FIGHTER_MARIO_LONG_JUMP_W : i32 = 0;
pub const FIGHTER_MARIO_LONG_JUMP_M : i32 = 1;
pub const FIGHTER_MARIO_LONG_JUMP_S : i32 = 2;
pub const FIGHTER_MARIO_LONG_JUMP_B : i32 = 3;

// Kirby
pub const FIGHTER_KIRBY_STATUS_ATTACK_LW3_FLAG_BOUNCE : i32 = 0x2100000B;

// Falco
pub const FIGHTER_FALCO_INSTANCE_WORK_ID_FLAG_KAA : i32 = 0x200000E5;

// Ganondorf
pub const FIGHTER_GANON_STATUS_WORK_ID_INT_TELEPORT_STEP : i32 = 0x11000008;
pub const FIGHTER_GANON_TELEPORT_STEP_INIT : i32 = 1;
pub const FIGHTER_GANON_TELEPORT_STEP_MOVE : i32 = 2;
pub const FIGHTER_GANON_TELEPORT_STEP_CHECK_FEINT : i32 = 3;
pub const FIGHTER_GANON_TELEPORT_STEP_END : i32 = 4;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_FEINT : i32 = 0x21000012;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_STOP : i32 = 0x21000013;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_OG_POS_X : i32 = 0x1000008;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_OG_POS_Y : i32 = 0x1000009;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_TELE_POS_X : i32 = 0x100000A;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLOAT_TELEPORT_TELE_POS_Y : i32 = 0x100000B;
pub const FIGHTER_GANON_STATUS_WORK_ID_FLAG_TELEPORT_START_GROUND : i32 = 0x21000014;

// Wario
pub const FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN : i32 = 0x100000C4;

// Toon Link
pub const FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED : i32 = 0x1000008;

// Lucario
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_IS_SPIRIT_BOMB : i32 = 0x200000E5;
pub const FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_IS_SUPER_DASH_CANCEL : i32 = 0x200000E6;

// Little Mac
pub static mut FUNNY_JUMPS : [i32; 8] = [10; 8];

// Shulk
pub const FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_PREV_DAMAGE : i32 = 0x53;
pub const FIGHTER_SHULK_INSTANCE_WORK_ID_FLOAT_BURST_RECOVER : i32 = 0x54;

// Ryu
pub static mut EX_FOCUS : [bool; 8] = [false; 8];
pub static mut EX_FLASH : [bool; 8] = [false; 8];
pub static mut SECRET_SENSATION : [bool; 8] = [false; 8];
pub static mut CAMERA : [bool; 8] = [false; 8];
pub static mut OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_Y : [f32; 8] = [0.0; 8];
pub static mut RYU_X : [f32; 8] = [0.0; 8];
pub static mut RYU_Y : [f32; 8] = [0.0; 8];
pub static mut SPECIAL_LW_TIMER : [i16; 8] = [-1; 8];
pub static mut SEC_SEN_TIMER : [f32; 8] = [-0.6; 8];
pub static mut OPPONENT_DIRECTION : [f32; 8] = [12.0; 8];
pub static mut VERT_EXTRA : [f32; 8] = [12.0; 8];
pub static mut SEC_SEN_STATE : [bool; 8] = [false; 8];
pub static mut SEC_SEN_DIREC : [i32; 8] = [0; 8];

// Ken
pub static mut QUICK_STEP_STATE : [i32; 8] = [0; 8];
/*
State list:
0 = Can Quick Step
2 = Cannot Quick Step
1 = Used to show you're in the first 22 frames of Quick Step.
*/
pub static mut STEP_KICK : [bool; 8] = [false; 8];
pub static mut VS1_CANCEL : [bool; 8] = [false; 8];
pub static mut V_SHIFT : [bool; 8] = [false; 8];
pub static mut V_TRIGGER : [bool; 8] = [false; 8];
pub static mut VT1_CANCEL : [bool; 8] = [false; 8];
pub static mut V_GAUGE : [i32; 8] = [0; 8];
pub static mut FLASH_MAX : [i32; 8] = [0; 8];
pub static mut FLASH_COUNTER : [i32; 8] = [0; 8];
pub static mut SHORYUREPPA : [i32; 8] = [0; 8];
pub static mut DIFF_X : [f32; 8] = [0.0; 8];
pub static mut DMG_RATIO : [f32; 8] = [0.8; 8];

// Corrin
pub static mut DRAGON_INSTALL : [f32; 8] = [0.0; 8];

// Incineroar
pub static mut REVENGE : [i32; 8] = [0; 8];

// Ridley
pub static mut FUNNY_RIDLEY : [bool; 8] = [false; 8];

// Joker
pub const FIGHTER_JACK_STATUS_WORK_ID_FLAG_SPECIAL_S_FEINT : i32 = 0x21000010;

// Terry
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT : i32 = 0x100000CD;
pub const FIGHTER_DOLLY_STATUS_WORK_ID_FLAG_SPECIAL_N_FEINT : i32 = 0x21000010;

// Sephiroth
// static mut CAN_WING : [bool; 8] = [true; 8];
// pub static mut ONE_WING : [f32; 8] = [-1.0; 8];

// Pyra
pub static mut CALLBACK : [i32; 8] = [0; 8];

// Yu Narukami
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S : i32 = 0x200000E2;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING : i32 = 0x200000E3;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY : i32 = 0x200000E4;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE : i32 = 0x200000E5;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_IS_EX : i32 = 0x200000E6;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT : i32 = 0x200000E7;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB : i32 = 0x200000E8;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND : i32 = 0x200000E9;
// pub const FIGHTER_YU_INSTANCE_WORK_ID_FLAG_TRAINING_TOOLS : i32 = 0x200000EA;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE : i32 = 0x4C;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP : i32 = 0x4D;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX : i32 = 0x4E;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY : i32 = 0x4F;
pub const FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_ROMAN_MOVE : i32 = 0x50;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL : i32 = 0x100000C0;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER : i32 = 0x100000C1;
pub const FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER : i32 = 0x100000C2;
pub const FIGHTER_YU_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION : i32 = 0x1100000A;
pub const FIGHTER_YU_SPECIAL_LW_FLAG_ROMAN_MOVE : i32 = 0x21000015;
pub const SP_1 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -6.0};
pub const SP_2 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: -2.0};
pub const SP_3 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 2.0};
pub const SP_4 : Vector3f = Vector3f{x: 0.0, y: 22.0, z: 6.0};
pub const SP_5 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: -2.0};
pub const SP_6 : Vector3f = Vector3f{x: 0.0, y: 27.0, z: 2.0};

#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = &mut *fighter.module_accessor;
        let id = entry_id(boma);
        // let kind = utility::get_kind(boma);

        if !smashball::is_training_mode() {
            FGC_TRAINING = false;
        }
        COUNTER_HIT_STATE[id] = 0;
        IS_DK[id] = false;
        CANCEL[id] = false;
        HIT_FRAME[id] = 0.0;
        BOUNCE[id] = false;
        SPECIAL_HITSTUN[id] = false;
        HIT_BY_SPECIAL_HITSTUN[id] = false;
        FGC_HITSTUN_MUL[id] = 1.2;
        SPECIAL_LW_TYPE[id] = 0;

        FUNNY_JUMPS[id] = 10;

        EX_FOCUS[id] = false;
        EX_FLASH[id] = false;
        SECRET_SENSATION[id] = false;
        CAMERA[id] = false;
        OPPONENT_X[id] = 0.0;
        OPPONENT_Y[id] = 0.0;
        RYU_X[id] = 0.0;
        RYU_Y[id] = 0.0;
        SPECIAL_LW_TIMER[id] = -1;
        SEC_SEN_TIMER[id] = -0.6;
        OPPONENT_DIRECTION[id] = 0.0;
        VERT_EXTRA[id] = 0.0;
        SEC_SEN_STATE[id] = false;
        SEC_SEN_DIREC[id] = 0;

        QUICK_STEP_STATE[id] = 0;
        STEP_KICK[id] = false;
        VS1_CANCEL[id] = false;
        V_SHIFT[id] = false;
        V_TRIGGER[id] = false;
        VT1_CANCEL[id] = false;
        V_GAUGE[id] = 0;
        FLASH_MAX[id] = 0;
        FLASH_COUNTER[id] = 0;
        SHORYUREPPA[id] = 0;
        DIFF_X[id] = 0.0;
        DMG_RATIO[id] = 0.0;

        DRAGON_INSTALL[id] = 0.0;

        REVENGE[id] = 0;

        FUNNY_RIDLEY[id] = false;

        CALLBACK[id] = 0;

        _TIME_COUNTER[id] = 0;
    }
}

pub fn install() {
    install_agent_resets!(
        fighter_reset
    );
}