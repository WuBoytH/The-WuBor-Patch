#![allow(non_upper_case_globals)]

use smash::phx::Vector3f;

// Common
pub const ZERO_VECTOR : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

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

pub mod fighter {
    pub mod instance {
        pub mod flag {
            pub const DISABLE_SPECIAL_N : i32 = 0x0000;
            pub const DISABLE_SPECIAL_S : i32 = 0x0001;
            pub const DISABLE_SPECIAL_HI : i32 = 0x0002;
            pub const DISABLE_SPECIAL_LW : i32 = 0x0003;
            // pub const GUARD_OFF_ATTACK_CANCEL : i32 = 0x0004;
            // pub const IS_FGC : i32 = 0x0005;
            // pub const DODGE_CANCEL : i32 = 0x0006;
            pub const JUMP_FROM_SQUAT : i32 = 0x0006;
            pub const SUPER_JUMP : i32 = 0x0007;
            pub const SUPER_JUMP_SET_MOMENTUM : i32 = 0x0008;
            // pub const FORCE_ESCAPE_AIR_SLIDE : i32 = 0x0008;
            pub const LEDGE_INTANGIBILITY : i32 = 0x0009;

            pub const PURGED : i32 = 0x000A;

            pub const CAN_LOOK_UP : i32 = 0x000B;

            pub const BURNOUT : i32 = 0x000C;
        }
        pub mod int {
            pub const TARGET_ID : i32 = 0x0000;
            pub const USED_GROUND_NORMALS : i32 = 0x0001;
            pub const USED_AERIALS : i32 = 0x0002;

            pub const PURGED_TIMER : i32 = 0x0003;
            pub const PURGED_EFF_HANDLE : i32 = 0x0004;

            // pub const CUSTOM_COMMAND_236_STEP : i32 = 0x0003;
            // pub const CUSTOM_COMMAND_236_TIMER : i32 = 0x0004;
            // pub const CUSTOM_COMMAND_214_STEP : i32 = 0x0005;
            // pub const CUSTOM_COMMAND_214_TIMER : i32 = 0x0006;
            // pub const CUSTOM_COMMAND_623_STEP : i32 = 0x0007;
            // pub const CUSTOM_COMMAND_623_TIMER : i32 = 0x0008;
            // pub const CUSTOM_COMMAND_236236_STEP : i32 = 0x0009;
            // pub const CUSTOM_COMMAND_236236_TIMER : i32 = 0x000A;
            pub const JUMP_FROM_SQUAT_COUNT_STATUS : i32 = 0x000B;

            pub const GUARD_CANCEL_PASS_FRAME : i32 = 0x000C;

            pub const BURNOUT_EFF_FRAME: i32 = 0x000D;
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
            pub const DASH_CANCEL : i32 = 0x1002;
            pub const SPECIAL_CANCEL : i32 = 0x1003;
            pub const ENABLE_AERIAL_STRING : i32 = 0x1004;
            pub const IS_DASH_CANCEL : i32 = 0x1005;
            pub const FORCE_ESCAPE_AIR_SLIDE_IN_STATUS : i32 = 0x1006;
            pub const SKIP_HITLAG_BUFFER_CHECK : i32 = 0x1007;
        }
        pub mod int {
            pub const ENABLED_AERIALS : i32 = 0x1000;
        }
        pub mod float {
            pub const HIT_FRAME : i32 = 0x1000;
        }

        pub const GUARD_CANCEL_ATTACK : i32 = 0x37D;
        pub const GUARD_CANCEL_ESCAPE_F : i32 = 0x37E;
        pub const GUARD_CANCEL_ESCAPE_B : i32 = 0x37F;
        pub const GUARD_CANCEL_PASS : i32 = 0x380;
        pub const GUARD_CANCEL_APPEAL : i32 = 0x381;
        pub const GUARD_CRUSH : i32 = 0x382;
        pub const COMMON_STATUS_EXTEND_MAX : i32 = 0x382;
    }
}

pub mod weapon {
    pub mod instance {
        pub mod flag {
            pub const FROM_POCKET : i32 = 0x0000;
        }
        pub mod int {
            pub const ORIGINAL_OWNER : i32 = 0x0000;
        }
    }
}

pub mod appeal {
    pub mod flag {
        pub const HOLD : i32 = 0x1050;
        pub const LOOP : i32 = 0x1051;
        pub const ENABLE_ACTION : i32 = 0x1052;
        pub const ENABLE_ACTION_IMM : i32 = 0x1053;
        pub const ACTION_BUTTON_CHECK : i32 = 0x1054;
        pub const ACTION_BUTTON_ENABLE_SUCCESS : i32 = 0x1055;
        pub const ACTION_BUFFER_SUCCESS : i32 = 0x1056;
        pub const ACTION_BUFFER_LOCKED : i32 = 0x1057;
    }
    pub mod int {
        pub const HOLD_BUTTON : i32 = 0x1050;
        pub const ACTION_BUTTON : i32 = 0x1051;
        pub const ACTION_FRAME : i32 = 0x1052;
        pub const RESTART_FRAME : i32 = 0x1053;
    }
    pub mod int64 {
        pub const ACTION_MOT : i32 = 0x1050;
        pub const LOOP_MOT : i32 = 0x1051;
    }
}

pub mod attack {
    pub mod flag {
        pub const INVALID_HOLD_INPUT : i32 = 0x1051;
    }
}

pub mod attack_air {
    pub mod flag {
        pub const ENABLE_LANDING_ATTACK : i32 = 0x1051;
    }
}

pub mod attack_dash {
    pub mod flag {
        pub const ENABLE_AIR_FALL : i32 = 0x1050;
        pub const ENABLE_AIR_CONTINUE : i32 = 0x1051;
        pub const ENABLE_GRAVITY : i32 = 0x1052;
        pub const GRAVITY_ENABLED : i32 = 0x1053;
        pub const ENABLE_AIR_LANDING : i32 = 0x1054;
    }
    pub mod float {
        pub const FALL_SPEED_Y_MUL : i32 = 0x1050;
    }
}

pub mod cliff {
    pub mod flag {
        pub const CLIFF_JUMP_BUTTON : i32 = 0x1050;
        pub const CLIFF_JUMP_MINI : i32 = 0x1051;
    }
}

pub mod damage_fly_roll {
    pub mod flag {
        pub const DISABLE_PASSIVE : i32 = 0x1050;
    }
}

pub mod dash {
    pub mod flag {
        pub const DISABLE_RUN : i32 = 0x1051;
    }
}

pub mod escape {
    pub mod flag {
        pub const DODGE_CANCEL : i32 = 0x1050;
    }
}

pub mod escape_air {
    pub mod flag {
        pub const SLIDE_ENABLE_ATTACK : i32 = 0x1050;
        pub const SLIDE_ENABLE_CANCEL : i32 = 0x1051;
    }
}

pub mod guard {
    pub mod flag {
        pub const ADD_BUFFER : i32 = 0x1050;
        pub const SET_SHIELD_LOW_SMOKE : i32 = 0x1051;
    }
    pub mod int {
        pub const SHIELD_EFF_ID : i32 = 0x1050;
        pub const GUARD_OFF_RESERVE_CAT1 : i32 = 0x1051;
        pub const JUST_SHIELD_COUNT : i32 = 0x1052;

        pub const DAMAGE_STOP_ESCAPE_STATUS : i32 = 0x1053;
        pub const JUST_SHIELD_XLU_COUNT : i32 = 0x1054;
    }
}

pub mod look_up {
    pub mod flag {
        pub const SHIFT_CAMERA : i32 = 0x1050;
    }
    pub mod int {
        pub const HOLD_COUNT : i32 = 0x1050;
    }
}

pub mod thrown {
    pub mod flag {
        pub const FORCE_LAUNCHED : i32 = 0x1051;
    }
}

pub mod bayonetta {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_AIR_S_D_IS_BOUNCE : i32 = 0x1150;
        }
    }
}

pub mod brave {
    pub mod instance {
        pub mod int {
            pub const NEXT_ROLL_INDEX : i32 = 0x0100;
            pub const USED_SPELL_MASK : i32 = 0x0101;
            pub const SPELL_SLOT_1 : i32 = 0x0102;
            pub const SPELL_SLOT_2 : i32 = 0x0103;
            pub const SPELL_SLOT_3 : i32 = 0x0104;
            pub const SPELL_SLOT_4 : i32 = 0x0105;
        }
    }
    pub mod status {
        pub mod flag {
            pub const ENABLE_CLEAR_PSYCHE_UP : i32 = 0x1150;
        }
    }
}

pub mod captain {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_S_ENABLE_MANUAL_ATTACK : i32 = 0x1150;
        }
    }
}

pub mod chrom {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_LW_CHANGE_KINETIC : i32 = 0x1150;
        }
    }
}

pub mod demon {
    pub mod status {
        pub mod flag {
            pub const ATTACK_STAND_2_SPECIAL_FINISHER : i32 = 0x1150;

            pub const ATTACK_STEP_2S_HOLD : i32 = 0x1150;
            pub const ATTACK_STEP_2S_CHECK_HOLD : i32 = 0x1151;
            pub const ATTACK_STEP_2S_TO_HEAVENS_GATE : i32 = 0x1152;
        }

        pub mod int {
            pub const ATTACK_STEP_2S_TO_HEAVENS_GATE_STEP : i32 = 0x1152;
        }
    }
}

pub mod dolly {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_CANCEL : i32 = 0x0100;
            pub const RISING_FORCE : i32 = 0x0101;
            pub const DISABLE_INPUT_SPECIAL_REVERSE : i32 = 0x0102;
        }
        pub mod int {
            pub const SUPER_SPECIAL_AURA : i32 = 0x0101;
            pub const SUPER_SPECIAL_AURA2 : i32 = 0x0102;

            pub const GUARD_CANCEL_ATTACK_VC_TYPE : i32 = 0x0103;
        }
        pub mod float {
            pub const GO_METER : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const DISABLE_METER_GAIN : i32 = 0x1100;
            pub const IS_SPECIAL_CANCEL : i32 = 0x1101;

            pub const ATTACK_DASH_COMMAND : i32 = 0x1150;

            pub const SPECIAL_F_CHECK_FEINT : i32 = 0x1150;
            pub const SPECIAL_F_FEINT : i32 = 0x1151;

            pub const SPECIAL_LW_BREAK : i32 = 0x1150;

            pub const SUPER_SPECIAL_TRY_TRIPLE : i32 = 0x1150;
            pub const SUPER_SPECIAL_CHECK_TRIPLE : i32 = 0x1151;
        }
        pub mod int {
            pub const ATTACK_DASH_STRENGTH : i32 = 0x1150;

            pub const SUPER_SPECIAL_TRIPLE_COUNT : i32 = 0x1150;
        }

        pub const ATTACK_LW32 : i32 = 0x203;
        pub const ATTACK_DASH_COMMAND : i32 = 0x204;
        pub const SPECIAL_N_COMMAND : i32 = 0x205;
        pub const SPECIAL_LW_BREAKING : i32 = 0x206;
    }
}

pub mod dolly_wave {
    pub mod status {
        pub mod flag {
            pub const FINAL_HIT : i32 = 0x1150;
        }
    }
}

pub mod edge {
    pub mod instance {
        pub mod int {
            pub const SPECIAL_HI_CANCEL_COUNT : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_HI_CANCEL : i32 = 0x1150;
        }
    }
}

pub mod edge_flare1 {
    pub mod instance {
        pub mod flag {
            pub const IS_GUARD_CANCEL : i32 = 0x0100;
        }
    }
}

pub mod element {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_LW_OUT_ATTACK : i32 = 0x0150;
            pub const SPECIAL_LW_OUT_ATTACK_FALL : i32 = 0x0151;
        }
    }
}

pub mod eflame_esword {
    pub mod status {
        pub mod flag {
            pub const ENABLE_EARLY_SPIN : i32 = 0x0150;
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
            pub const TELEPORT_FEINT : i32 = 0x1150;
            pub const TELEPORT_STOP : i32 = 0x1151;
            pub const TELEPORT_START_GROUND : i32 = 0x1152;
        }
        pub mod int {
            pub const TELEPORT_STEP : i32 = 0x1150;
        }
        pub mod float {
            pub const TELEPORT_START_POS: i32 = 0x1150;
            pub const TELEPORT_END_POS: i32 = 0x1152;
        }
    }

    pub const TELEPORT_STEP_INIT : i32 = 1;
    pub const TELEPORT_STEP_MOVE : i32 = 2;
    pub const TELEPORT_STEP_MOVE_DONE : i32 = 3;
    pub const TELEPORT_STEP_CHECK_FEINT : i32 = 4;
    pub const TELEPORT_STEP_END : i32 = 5;
}

pub mod gaogaen {
    pub mod status {
        pub mod flag {
            pub const REVENGE_AUTO : i32 = 0x1150;
            pub const REVENGE_CRITICAL : i32 = 0x1151;
        }
    }
}

pub mod ike {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_N_AIR : i32 = 0x1150;
            pub const SPECIAL_N_RANGED_ERUPTION : i32 = 0x1151;
            pub const SPECIAL_N_ENABLE_CRITICAL : i32 = 0x1152;
        }
        pub mod int {
            pub const ERUPTION_COUNT : i32 = 0x1150;
        }
        pub mod float {
            pub const SPECIAL_N_ERUPT_LOCATION : i32 = 0x1150;
        }
    }
}

pub mod jack {
    pub mod instance {
        pub mod flag {
            pub const SPECIAL_LW_CANCEL : i32 = 0x0150;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_N_FIRST : i32 = 0x1150;

            pub const SPECIAL_S_FEINT : i32 = 0x1150;
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

pub mod kirby {
    pub mod status {
        pub mod flag {
            pub const ATTACK_LW3_BOUNCE : i32 = 0x1150;
        }
        pub mod int {
            pub const APPEAL_S_LOOP_COUNT : i32 = 0x1150;
        }

        pub const DOLLY_SPECIAL_N_COMMAND : i32 = super::super::fighter::status::COMMON_STATUS_EXTEND_MAX + 1;
    }
}

pub mod koopa_breath {
    pub mod status {
        pub const HIT : i32 = 0x1;
    }
}

pub mod lucario {
    pub mod instance {
        pub mod flag {
            pub const USED_AURA_CHARGE_AIR : i32 = 0x0100;
            pub const FORCE_LANDING_FALL_SPECIAL : i32 = 0x0101;
            pub const EXTREME_SPEED_FORCE_NO_AURA : i32 = 0x0102;
        }
        pub mod int {
            pub const AURA_LEVEL : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_N_ENABLE_SUPERDASH : i32 = 0x1150;
            pub const SPECIAL_N_SPIRIT_BOMB : i32 = 0x1151;
            pub const SPECIAL_N_START_FROM_GROUND : i32 = 0x1152;
            pub const SPECIAL_N_SPIRIT_BOMB_ENABLE_FALL : i32 = 0x1153;
            pub const SPECIAL_N_SPIRIT_BOMB_FALLING : i32 = 0x1154;

            pub const SPECIAL_S_CHECK_ENHANCE : i32 = 0x1150;
            pub const SPECIAL_S_SET_MOTION : i32 = 0x1151;
            pub const SPECIAL_S_ENHANCE : i32 = 0x1152;
            pub const SPECIAL_S_ENABLE_GRAVITY : i32 = 0x1153;
            pub const SPECIAL_S_POST_GRAVITY : i32 = 0x1154;

            pub const SPECIAL_HI_CANCELLED_INTO : i32 = 0x1150;
            pub const SPECIAL_HI_SUPER_DASH_CANCEL : i32 = 0x1151;

            pub const SPECIAL_LW_ENABLE_CANCEL : i32 = 0x1150;
            pub const SPECIAL_LW_ATTACK : i32 = 0x1151;
            pub const SPECIAL_LW_CANCEL : i32 = 0x1152;
            pub const SPECIAL_LW_CANCEL_FORCE_JUMP : i32 = 0x1153;
            pub const SPECIAL_LW_CHARGE_END : i32 = 0x1154;
        }
        pub mod int {
            pub const AURA_ENHANCED_BY : i32 = 0x1100;

            pub const SPECIAL_LW_STEP : i32 = 0x1150;
            pub const SPECIAL_LW_CHARGE_TIME : i32 = 0x1151;
            pub const SPECIAL_LW_CHARGES_GAINED : i32 = 0x1152;
            pub const SPECIAL_LW_EFF1 : i32 = 0x1153;
            pub const SPECIAL_LW_EFF2 : i32 = 0x1154;
            pub const SPECIAL_LW_EFF3 : i32 = 0x1155;
        }
    }

    pub const SPECIAL_LW_STEP_START : i32 = 0x0;
    pub const SPECIAL_LW_STEP_CHARGE : i32 = 0x1;
    pub const SPECIAL_LW_STEP_END : i32 = 0x2;
}

pub mod lucario_auraball {
    pub mod instance {
        pub mod flag {
            pub const SPIRIT_BOMB : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const FROM_AIR : i32 = 0x1100;
            pub const EXPLOSION : i32 = 0x1101;
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
        }
        pub mod int {
            pub const SP_LEVEL : i32 = 0x0100;
            pub const SP_EFFECT_TIMER : i32 = 0x0101;
            pub const SP_FLASH_TIMER : i32 = 0x0102;
            pub const SP_GLOW_TIMER : i32 = 0x0103;
            pub const SHADOW_EFF_ID : i32 = 0x0104;
            pub const SP_GAIN_PENALTY : i32 = 0x0105;
        }
        pub mod float {
            pub const SP_GAUGE : i32 = 0x0100;
            pub const SPENT_SP : i32 = 0x0101;
            pub const SP_GAUGE_MAX : i32 = 0x0102;
        }
    }
    pub mod status {
        pub mod flag {
            pub const IS_EX : i32 = 0x1100;
            pub const CAN_ONE_MORE : i32 = 0x1101;

            pub const ATTACK_DASH_BIG_GAMBLE : i32 = 0x1150;
            pub const ATTACK_DASH_BIG_GAMBLE_TRANSITION : i32 = 0x1151;

            pub const SPECIAL_LW_DECIDE_ROMAN_DIREC : i32 = 0x1150;
            pub const SPECIAL_LW_ROMAN_MOVE : i32 = 0x1151;
        }
        pub mod float {
            pub const SPECIAL_LW_ROMAN_MOVE : i32 = 0x1150;
        }

        pub const SPECIAL_N_COMMAND       : i32 = 0x1EB + 0;
        pub const SPECIAL_S_COMMAND       : i32 = 0x1EB + 1;
        pub const SPECIAL_HI_COMMAND      : i32 = 0x1EB + 2;
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
            pub const BONKER : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const ATTACK_AIR_F_HOLD : i32 = 0x1150;

            pub const SPECIAL_S_TRY_HOP : i32 = 0x1150;
            pub const SPECIAL_S_HOP : i32 = 0x1151;
            pub const SPECIAL_S_ENABLE_CONTROL : i32 = 0x1152;

            pub const SPECIAL_LW_IS_BLJ : i32 = 0x1150;
        }

        pub const SPECIAL_LW_JUMP : i32 = 0x1E3;
        pub const SPECIAL_LW_LANDING : i32 = 0x1E4;
        pub const SPECIAL_AIR_LW_START : i32 = 0x1E5;
        pub const SPECIAL_AIR_LW_FALL : i32 = 0x1E6;
        pub const SPECIAL_AIR_LW_LANDING : i32 = 0x1E7;
        pub const SPECIAL_AIR_LW_CANCEL : i32 = 0x1E8;
    }
}

pub mod mariod {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_N_ENABLE_ACTIONS : i32 = 0x1150;
        }
    }
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

            pub const ATTACK_3_CHANGE_MOTION : i32 = 0x1150;
            pub const ATTACK_F3_HEAVY : i32 = 0x1151;

            pub const SPECIAL_S_ENABLE_SPECIALS : i32 = 0x1150;
            pub const SPECIAL_S_DASH : i32 = 0x1151;
            pub const SPECIAL_S_END : i32 = 0x1152;
            pub const SPECIAL_S2_FINAL_BLOW : i32 = 0x1153;
        }
        pub mod int {
            pub const SPECIAL_S2_START_SITUATION : i32 = 0x1150;
            pub const SPECIAL_S2_LOOP_COUNT : i32 = 0x1151;
        }
        pub mod float {
            pub const SPECIAL_S_ANGLE : i32 = 0x1150;
        }

        pub const STANCE_ENTER            : i32 = 0x1EB + 0;
        pub const STANCE_WAIT             : i32 = 0x1EB + 1;
        pub const STANCE_SQUAT            : i32 = 0x1EB + 2;
        pub const STANCE_SQUAT_WAIT       : i32 = 0x1EB + 3;
        pub const STANCE_SQUAT_RV         : i32 = 0x1EB + 4;
        pub const STANCE_EXIT             : i32 = 0x1EB + 5;
        pub const STANCE_DASH_F           : i32 = 0x1EB + 6;
        pub const STANCE_DASH_B           : i32 = 0x1EB + 7;
        pub const STANCE_ATTACK           : i32 = 0x1EB + 8;
        pub const STANCE_ATTACK_LW3       : i32 = 0x1EB + 9;
        pub const STANCE_ATTACK_LW4       : i32 = 0x1EB + 10;
        pub const STANCE_ATTACK_HI3       : i32 = 0x1EB + 11;
        pub const STANCE_ATTACK_B3        : i32 = 0x1EB + 12;
        pub const STANCE_ATTACK_F3        : i32 = 0x1EB + 13;
        pub const STANCE_SPECIAL_S        : i32 = 0x1EB + 14;
        pub const STANCE_SPECIAL_S_DASH   : i32 = 0x1EB + 15;
        pub const STANCE_SPECIAL_S_END    : i32 = 0x1EB + 16;
        pub const STANCE_SPECIAL_S2_START : i32 = 0x1EB + 17;
        pub const STANCE_SPECIAL_S2_LOOP  : i32 = 0x1EB + 18;
        pub const STANCE_SPECIAL_S2_END   : i32 = 0x1EB + 19;
        pub const STANCE_SPECIAL_S2_END2  : i32 = 0x1EB + 20;
        // pub const STANCE_SPECIAL_S2_LANDING : i32 = 21;
    }
}

pub mod mewtwo {
    pub mod status {
        pub mod float {
            pub const SPECIAL_HI_2_SPEED_X : i32 = 0x1150;
            pub const SPECIAL_HI_2_SPEED_Y : i32 = 0x1151;
        }
    }
}

pub mod packun {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_HI_ENABLE_CANCEL : i32 = 0x1100;
        }
    }
}

pub mod palutena {
    pub mod status {
        pub mod float {
            pub const SPECIAL_HI_2_SPEED_X : i32 = 0x1150;
            pub const SPECIAL_HI_2_SPEED_Y : i32 = 0x1151;
        }
    }
}

pub mod pikachu {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_LW_START : i32 = 0x1150;
            pub const SPECIAL_LW_ENABLE_LANDING : i32 = 0x1151;
            pub const SPECIAL_LW_ENABLE_GRAVITY : i32 = 0x1152;
        }
    }
}

pub mod pikachu_dengekidama {
    pub mod status {
        pub mod flag {
            pub const SPEED_UP : i32 = 0x1150;
        }
    }
}

pub mod pikmin {
    pub mod instance {
        pub mod flag {
            pub const ATTACK_HI3_LANDING : i32 = 0x0100;
        }
        pub mod int {
            pub const ATTACK_S3_LOOP_COUNT : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const ATTACK_HI3_DRIFT : i32 = 0x1150;

            pub const GUARD_CANCEL_ATTACK_SHOOT : i32 = 0x1150;
            pub const GUARD_CANCEL_ATTACK_DONE : i32 = 0x1151;
        }
        pub mod int {
            pub const ATTACK_S3_STEP : i32 = 0x1150;
        }
    }
    pub const ATTACK_S3_STEP_START : i32 = 0;
    pub const ATTACK_S3_STEP_LOOP : i32 = 1;
    pub const ATTACK_S3_STEP_END : i32 = 2;
}

pub mod pikmin_pikmin {
    pub mod status {
        pub const GUARD_CANCEL_ATTACK_START : i32 = 0x44;
        pub const GUARD_CANCEL_ATTACK : i32 = 0x45;
    }
}

pub mod richter {
    pub mod instance {
        pub mod int {
            pub const AXE_ID : i32 = 0x0100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const ATTACK_JUST_INPUT : i32 = 0x1150;

            pub const ATTACK_AIR_LW_IGNORE_BOUNCE : i32 = 0x1150;

            pub const SPECIAL_N_SHOOT : i32 = 0x1150;
        }
    }
}

pub mod rockman {
    pub mod instance {
        pub mod flag {
            pub const CHARGE_SHOT_CHARGING : i32 = 0x0100;
            pub const CHARGE_SHOT_PLAYED_FX : i32 = 0x0101;
            pub const CHARGE_SHOT_RELEASE : i32 = 0x0102;
        }
        pub mod int {
            pub const CHARGE_SHOT_FRAME : i32 = 0x0100;
            pub const CHARGE_SHOT_EFF_HANDLE : i32 = 0x0101;
            pub const CHARGE_SHOT_SND_HANDLE : i32 = 0x0102;
            pub const CHARGE_SHOT_RELEASE_FRAME : i32 = 0x0103;
        }
    }
    pub mod status {
        pub mod flag {
            pub const CHARGE_SHOT_KEEP_CHARGE : i32 = 0x1100;
        }
        pub mod int {
            pub const AIR_SHOOTER_NUM : i32 = 0x1100;
        }
    }
}

pub mod rockman_airshooter {
    pub mod instance {
        pub mod int {
            pub const NUM : i32 = 0x1100;
        }
    }
    pub mod status {
        pub mod flag {
            pub const MOVE : i32 = 0x1100;
        }
    }
}

pub mod rosetta_tico {
    pub mod status {
        pub const GUARD_CANCEL_ATTACK : i32 = 0x33;
    }
}

pub mod ryu {
    pub mod instance {
        pub mod flag {
            pub const DENJIN_CHARGE : i32 = 0x0100;
            pub const DENJIN_RUSH_INHERIT : i32 = 0x0101;
            pub const SKIP_FINAL_PROX_CHECK : i32 = 0x0102;
        }
        pub mod int {
            pub const DENJIN_EFF_HANDLE : i32 = 0x0100;
            pub const RUSH_VC_TYPE : i32 = 0x0101;
            pub const IMPACT_PUNISH_VC_TYPE : i32 = 0x0102;
        }
    }
    pub mod status {
        pub mod flag {
            pub const USED_DENJIN_CHARGE : i32 = 0x1100;
            pub const SET_DENJIN_AURA : i32 = 0x1101;

            pub const SPECIAL_HI_SPECIAL_EFFECT : i32 = 0x1102;

            pub const SPECIAL_LW_RUSH_RESUME_ENERGY : i32 = 0x1102;
            pub const SPECIAL_LW_RUSH_ENABLE_ATTACK : i32 = 0x1103;

            pub const SPECIAL_LW_IMPACT_HIT : i32 = 0x1102;
            pub const SPECIAL_LW_IMPACT_SHIELD : i32 = 0x1103;
            pub const SPECIAL_LW_IMPACT_JUST_SHIELD : i32 = 0x1104;
            pub const SPECIAL_LW_IMPACT_SHIELD_WIND : i32 = 0x1105;
            pub const SPECIAL_LW_IMPACT_ENABLED_ARMOR : i32 = 0x1106;
            pub const SPECIAL_LW_IMPACT_REMOVE_ARMOR : i32 = 0x1107;

            pub const SPECIAL_DECIDE_STRENGTH : i32 = 0x1150;
        }
        pub mod int {
            pub const GUARD_SPECIAL_LW_KIND : i32 = 0x1100;
        }
    }

    pub const GUARD_SPECIAL_LW_KIND_IMPACT : i32 = 0x0;
    pub const GUARD_SPECIAL_LW_KIND_REVERSAL : i32 = 0x1;
}

pub mod ryu_shinkuhadoken {
    pub mod status {
        pub const FINISH : i32 = 0x2;
    }
}

pub mod ken {
    pub mod instance {
        pub mod flag {
            pub const QUICK_STEP_INHERIT : i32 = 0x0100;

            pub use super::super::super::ryu::instance::flag::SKIP_FINAL_PROX_CHECK;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_N2_GROUND_BRANCH_CHECK : i32 = 0x1100;
            pub const SPECIAL_N2_GROUND_BRANCH_LM : i32 = 0x1101;
            pub const SPECIAL_N2_GROUND_BRANCH_H : i32 = 0x1102;

            pub const SPECIAL_N2_AIR_DISABLE_GRAVITY : i32 = 0x1100;
            pub const SPECIAL_N2_AIR_ENABLE_LANDING : i32 = 0x1101;

            pub use super::super::super::ryu::status::flag::SPECIAL_HI_SPECIAL_EFFECT;

            pub const SPECIAL_LW_ENABLE_ACTION : i32 = 0x1100;
            pub const SPECIAL_LW_UNABLE_ACTION : i32 = 0x1101;
            pub const SPECIAL_LW_ENABLED_ACTION : i32 = 0x1102;
            pub const SPECIAL_LW_RESET_GRAVITY : i32 = 0x1103;

            pub use super::super::super::ryu::status::flag::SPECIAL_DECIDE_STRENGTH;
            pub const QUICK_STEP_INHERITED : i32 = 0x1151;
        }
        pub mod int64 {
            pub const SPECIAL_N2_GROUND_BRANCH_MOTION : i32 = 0x1100;
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
    pub mod status {
        pub mod flag {
            pub const ATTACK_AIR_N_START_FLOAT : i32 = 0x1150;

            pub const SPECIAL_LW_BOUNCE : i32 = 0x1150;
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

pub mod simon {
    pub use super::richter::*;
}

pub mod snake_c4 {
    pub mod instance {
        pub mod flag {
            pub const TRACKER_ENABLED : i32 = 0x0150;
            pub const TRACKER_VISIBLE : i32 = 0x0151;
        }
        pub mod int {
            pub const TRACKER_EFF : i32 = 0x0150;
        }
    }
}

pub mod sonic {
    pub mod status {
        pub mod flag {
            pub const ENABLE_TRICK : i32 = 0x1100;

            pub const DASH_FROM_THROW_LW : i32 = 0x1150;

            pub const SPECIAL_AIR_S_FIRST : i32 = 0x1150;
            pub const SPECIAL_AIR_S_CHECK_END : i32 = 0x1151;
            pub const SPECIAL_AIR_S_TO_END : i32 = 0x1152;
            pub const SPECIAL_AIR_S_CHECK_HIT : i32 = 0x1153;

            pub const SPECIAL_AIR_LW_TO_LOOP : i32 = 0x1150;
            pub const SPECIAL_AIR_LW_HIT : i32 = 0x1151;
            pub const SPECIAL_AIR_LW_BOUND_START : i32 = 0x1152;
            pub const SPECIAL_AIR_LW_BOUNDING : i32 = 0x1153;
            pub const SPECIAL_AIR_LW_BOUND_IS_GROUND : i32 = 0x1154;
            pub const SPECIAL_AIR_LW_ESCAPE_AIR_RESERVE : i32 = 0x1155;
            pub const SPECIAL_AIR_LW_SPECIAL_N_FALL_RESERVE : i32 = 0x1156;
            pub const SPECIAL_AIR_LW_SPECIAL_S_RESERVE : i32 = 0x1157;
            pub const SPECIAL_AIR_LW_FROM_TRICK : i32 = 0x1158;

            pub const TRICK_ENABLE_MOVEMENT : i32 = 0x1150;
            pub const TRICK_ENABLE_CONTROL : i32 = 0x1151;
        }
        pub mod int {
            pub const SPECIAL_AIR_S_HOLD_COUNT_REMAIN : i32 = 0x1150;

            pub const SPECIAL_AIR_LW_JUMP_RESERVE : i32 = 0x1150;
        }
        pub mod float {
            pub const SPECIAL_AIR_LW_BOUND_SPEED_X : i32 = 0x1150;
            pub const SPECIAL_AIR_LW_BOUND_SPEED_Y : i32 = 0x1151;
            pub const SPECIAL_AIR_LW_BOUND_ANGLE : i32 = 0x1152;
        }
        pub const LOOK_UP : i32 = 0x1F3;
        pub const LOOK_UP_WAIT : i32 = 0x1F4;
        pub const LOOK_UP_RV : i32 = 0x1F5;
        pub const SPECIAL_AIR_S_START : i32 = 0x1F6;
        pub const SPECIAL_AIR_S_HOLD : i32 = 0x1F7;
        pub const SPECIAL_AIR_S_END : i32 = 0x1F8;
        pub const SPECIAL_AIR_LW_START : i32 = 0x1F9;
        pub const SPECIAL_AIR_LW_LOOP : i32 = 0x1FA;
        pub const SPECIAL_AIR_LW_BOUND : i32 = 0x1FB;
        pub const TRICK : i32 = 0x1FC;
    }
}

pub mod szerosuit {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_HI_DECIDE_MOTION : i32 = 0x1150;
        }
    }
}

pub mod toonlink {
    pub mod status {
        pub mod flag {
            pub const ATTACK_AIR_LW_BOUNCE : i32 = 0x1150;

            pub const SPECIAL_HI_MOVE : i32 = 0x1150;
        }
    }
}

pub mod wario {
    pub mod status {
        pub mod flag {
            pub const THROW_B_MOVE : i32 = 0x1150;
            pub const THROW_B_CONTROL_RESET : i32 = 0x1151;
        }
    }
}

pub mod yoshi {
    pub mod status {
        pub mod flag {
            pub const SPECIAL_S_HOP : i32 = 0x1150;

            pub const SPECIAL_HI_SPECIAL_HOLD : i32 = 0x1150;
            pub const SPECIAL_HI_RISE : i32 = 0x1151;
            pub const SPECIAL_HI_RISE_CUT : i32 = 0x1152;
            pub const SPECIAL_HI_RISE_AIR_CANCEL : i32 = 0x1153;
            pub const SPECIAL_HI_RISE_GROUND_CANCEL : i32 = 0x1154;
        }
        pub mod int {
            pub const SPECIAL_HI_START_SITUATION : i32 = 0x1150;
        }
    }
}

pub mod zelda {
    pub mod status {
        pub mod float {
            pub const SPECIAL_HI_2_SPEED_X : i32 = 0x1150;
            pub const SPECIAL_HI_2_SPEED_Y : i32 = 0x1151;
        }
    }
}