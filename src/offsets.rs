//! Every hardcoded executable offset used by the main NRO lives here.
//!
//! All values are relative to the start of the game's `.text` region unless
//! noted otherwise (item offsets are relative to the `item` NRO's base and are
//! resolved when that NRO loads). The `+ 0x5b0`, `- 0x1A0`, `+ 0x450` and
//! `- 0xE0` adjustments are the shifts between the version the offset was
//! originally found in and the version the patch currently targets, and are
//! kept explicit so the original address can still be looked up.
//!
//! Layout:
//! - `fighter::<name>`  functions belonging to a fighter's code
//! - `fighter::<name>::vtable`  addresses inside that fighter's vtable(s)
//! - `weapon::<name>`   functions belonging to an article / weapon
//! - `item::<name>`     item NRO relative offsets
//! - `system::<area>`   engine, controller, modules and other shared code

#![allow(dead_code)]

pub mod fighter {
    /// Shared fighter code that more than one character hooks into.
    pub mod common {
        /// Autoturn handler shared by Dolly / Ryu / Ken (`dolly_transition_handler`, `ryu_ken_autoturn_handler`).
        pub static mut AUTOTURN_HANDLER : usize = 0x69ae40;
        /// Final Smash cancel check shared by Dolly / Ryu / Ken (`dolly_what_is_this`, `ryu_ken_check_final_can_cancel`).
        pub static mut CHECK_FINAL_CAN_CANCEL : usize = 0x695c80;
        /// Sets jostle team for all fighters to 0.
        pub static mut JOSTLE_TEAM_PATCH : usize = 0x60eb08;
    }

    pub mod belmont {
        pub static mut ON_INIT_DEATH : usize = 0x11944e0;
        pub static mut SIMON_ON_DAMAGE : usize = 0x1195890;

        pub mod vtable {
            pub static mut SIMON_START : usize = 0x5041690;
        }
    }

    pub mod brave {
        pub static mut HANDLE_PSYCHE_UP_HIT : usize = 0x853ce0;
        pub static mut REMOVE_PSYCHE_UP : usize = 0x853e10;
        /// Removes a Psyche Up check.
        pub static mut PSYCHE_UP_CHECK_PATCH : usize = 0x8542ec;
    }

    pub mod captain {
        pub static mut ON_ATTACK_ORIGINAL : usize = 0x8b8b90;

        pub mod vtable {
            pub static mut ON_FRAME : usize = 0x4f99688;
            pub static mut ON_ATTACK : usize = 0x4f99740;
            pub static mut ON_DAMAGE : usize = 0x4f99840;
        }
    }

    pub mod demon {
        pub static mut SOME_EVENT : usize = 0x934310;
    }

    pub mod diddy {
        pub static mut GET_SPECIAL_S_OFFSET : usize = 0x721240;
    }

    pub mod dolly {
        pub static mut PER_FRAME : usize = 0x971490;
        pub static mut CHECK_SUPER_SPECIAL : usize = 0x971250;
        pub static mut HANDLE_SPECIAL_COMMAND_TURNAROUND : usize = 0x972db0;
        pub static mut ON_ATTACK_INNER : usize = 0x9720a0;
        pub static mut RESET2 : usize = 0x970e20;
        /// Max Status Terms?
        pub static mut MAX_STATUS_TERMS : usize = 0x4fa7e40;
        /// Some kind of transition check table (bytes at +0x203..=0x205 are patched).
        pub static mut TRANSITION_CHECK_TABLE : usize = 0x4fa7e70;

        pub mod vtable {
            pub static mut ON_ATTACK : usize = 0x4fa7a28;
        }
    }

    pub mod eflame {
        pub static mut HAS_ESWORD_ENABLE_TRANSITIONS : usize = 0xa0c250;
        pub static mut HAS_ESWORD_DISABLE_STATUSES : usize = 0xa0c880;
    }

    pub mod elight {
        /// Disables Foresight.
        pub static mut FORESIGHT_PATCH_1 : usize = 0xa28e78;
        pub static mut FORESIGHT_PATCH_2 : usize = 0xa28e84;
    }

    pub mod gamewatch {
        pub static mut CHANGE_STATUS_CALLBACK : usize = 0xa83010;
    }

    pub mod ganon {
        pub static mut LINK_EVENT : usize = 0xaa6990;
        pub static mut STATUS_TRANSITION : usize = 0xaa6800;
    }

    pub mod gaogaen {
        pub static mut ON_ATTACK : usize = 0xab9970;
        /// Skips to the end of the Revenge check after changing statuses.
        pub static mut REVENGE_CHECK_SKIP_PATCH : usize = 0xab9ff0;
    }

    pub mod iceclimber {
        /// Patches out disabling grabs.
        pub static mut DISABLE_GRAB_PATCH_1 : usize = 0xfba284;
        pub static mut DISABLE_GRAB_PATCH_2 : usize = 0xfb9780;
        pub static mut DISABLE_GRAB_PATCH_3 : usize = 0xfb9794;
        pub static mut DISABLE_GRAB_PATCH_4 : usize = 0xfb97a8;
        /// No more cheering or crying.
        pub static mut CHEER_CRY_PATCH_1 : usize = 0xfb60d4;
        pub static mut CHEER_CRY_PATCH_2 : usize = 0xfba298;
        pub static mut CHEER_CRY_PATCH_3 : usize = 0xfba52c;
        pub static mut CHEER_CRY_PATCH_4 : usize = 0xfba850;
        /// A special patch, to see who's the fastest.
        pub static mut SPEED_PATCH_1 : usize = 0x2f7dcc;
        pub static mut SPEED_PATCH_2 : usize = 0x2f7e94;
    }

    pub mod ike {
        pub static mut ON_ATTACK : usize = 0xaf9350;

        pub mod vtable {
            pub static mut START : usize = 0x4fc2940;
        }
    }

    pub mod jack {
        pub static mut CUSTOMIZER : usize = 0xb2f820;
        pub static mut CHECK_DOYLE_SUMMON_DISPATCH : usize = 0xb30954;
        pub static mut DAMAGE_CALLBACK : usize = 0xb36ee0;
        pub static mut DAMAGE_CALLBACK_2 : usize = 0xb36ea0;
        pub static mut DAMAGE_CALLBACK_3 : usize = 0x21b3440;
        pub static mut HANDLE_GUN_DODGE_STALING : usize = 0xb34450;
        pub static mut CALL_SUMMON_DISPATCH : usize = 0x21b35f0;
        pub static mut ON_ATTACK_INNER : usize = 0xb33d30;
        pub static mut ON_GRAB : usize = 0xb33820;
        /// Disables passive meter gain.
        pub static mut PASSIVE_METER_GAIN_PATCH : usize = 0xb31620;
        /// Disables automatically summoning Arsene.
        pub static mut AUTO_SUMMON_ARSENE_PATCH_1 : usize = 0xb3153c;
        pub static mut AUTO_SUMMON_ARSENE_PATCH_2 : usize = 0xb30dd4;

        pub mod vtable {
            pub static mut ON_ATTACK : usize = 0x4fc71b8;
        }
    }

    pub mod kirby {
        pub static mut LOSE_COPY_ABILITY : usize = 0xb96770;
        pub static mut FRAME_BRANCH_COPY_VTABLE : usize = 0xb97c78;
    }

    pub mod koopa {
        pub static mut PER_FRAME : usize = 0xbc2290;
    }

    pub mod lucario {
        pub static mut CHECK_AURA : usize = 0xc5c010;
        pub static mut CHECK_AURA_2 : usize = 0xc5be40;
        pub static mut HANDLE_AURA : usize = 0xc5e550;
        pub static mut HANDLE_AURA_2 : usize = 0xc5e6f0;
        pub static mut ON_GRAB : usize = 0xc5d5a0;
        pub static mut SET_EFFECT_SCALE : usize = 0xc5ce40;
    }

    pub mod lucina {
        pub static mut MARTH_LUCINA_ON_ATTACK : usize = 0xcd9ea0;

        pub mod vtable {
            pub static mut INIT : usize = 0x4fe7fc0;
            pub static mut PER_FRAME : usize = 0x4fe8008;
            pub static mut ON_ATTACK : usize = 0x4fe80c0;
        }
    }

    pub mod luigi {
        pub static mut CHANGE_MOTION_CALLBACK : usize = 0xca1510;
        pub static mut LINK_EVENT : usize = 0xca0e70;
    }

    pub mod mariod {
        pub static mut INIT : usize = 0xcc8f20;
        pub static mut ON_STATUS_CHANGE : usize = 0xcc9770;
    }

    pub mod ness {
        pub static mut INIT : usize = 0xdefdf0;
    }

    pub mod pickel {
        pub static mut IS_MINING_MATERIAL_TABLE_NORMAL : usize = 0xf10680;
        pub static mut GET_MINING_MATERIAL_TABLE_RESULT : usize = 0xf10710;
        /// Forces the common mining pattern.
        pub static mut FORCE_COMMON_MINING_PATTERN_PATCH : usize = 0xf1078c;
        /// Skips the check for what tool to mine with.
        pub static mut SKIP_MINING_TOOL_CHECK_PATCH : usize = 0xf0f794;
        /// Forces the mining tool used to be Pickaxe.
        pub static mut FORCE_PICKAXE_PATCH : usize = 0xf0f820;
        /// Removes the link event setting the crafting table auto respawn timer.
        pub static mut CRAFTING_TABLE_AUTO_RESPAWN_LINK_EVENT_PATCH : usize = 0xf0c20c;
        /// Disables the count_down_int for the crafting table auto respawn timer.
        pub static mut CRAFTING_TABLE_AUTO_RESPAWN_COUNT_DOWN_PATCH : usize = 0xf088c8;
        /// Skips the crafting table auto respawn create table call.
        pub static mut CRAFTING_TABLE_AUTO_RESPAWN_CREATE_PATCH : usize = 0xf088cc;
    }

    pub mod reflet {
        /// Always start a match with Levin Sword.
        pub static mut LEVIN_SWORD_START_PATCH : usize = 0x1005d30;
        /// Fixes an issue causing Robin to be un-trippable after using Thoron.
        pub static mut THORON_TRIP_FIX_PATCH : usize = 0x1009970;
    }

    pub mod rockman {
        pub static mut VTABLE_FUNC : usize = 0x107e970;
        pub static mut DO_LEAFSHIELD_THINGS_DISABLE : usize = 0x1083bec;
        pub static mut DO_LEAFSHIELD_THINGS_ENABLE : usize = 0x10838e0;
        /// Forces the original Leaf Shield handler to not run so we can run the custom one.
        pub static mut LEAFSHIELD_HANDLER_SKIP_PATCH : usize = 0x107eaa4;
        /// Removes the check that forces the removal of Leaf Shield if you are not within certain statuses.
        pub static mut LEAFSHIELD_REMOVAL_CHECK_PATCH : usize = 0x107ff6c;
        /// Manual Leaf Shield "disable" checks, stubbed so FighterSpecializer_Rockman::is_leafshield is used instead.
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_1 : usize = 0x1083bec;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_2 : usize = 0x1083c0c;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_3 : usize = 0x1083c28;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_4 : usize = 0x1083c3c;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_5 : usize = 0x1083c50;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_6 : usize = 0x1083c6c;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_7 : usize = 0x1083c80;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_8 : usize = 0x1083c94;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_9 : usize = 0x1083ca8;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_10 : usize = 0x1083cbc;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_11 : usize = 0x1083cd0;
        pub static mut LEAFSHIELD_DISABLE_CHECK_PATCH_12 : usize = 0x1083ce4;
        /// Manual Leaf Shield "enable" checks, stubbed so FighterSpecializer_Rockman::is_leafshield is used instead.
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_1 : usize = 0x10838e0;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_2 : usize = 0x1083900;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_3 : usize = 0x1083928;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_4 : usize = 0x1083944;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_5 : usize = 0x1083958;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_6 : usize = 0x108396c;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_7 : usize = 0x1083988;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_8 : usize = 0x108399c;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_9 : usize = 0x10839b0;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_10 : usize = 0x10839c4;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_11 : usize = 0x10839d8;
        pub static mut LEAFSHIELD_ENABLE_CHECK_PATCH_12 : usize = 0x10839ec;
        /// Patches which status to compare to for Metal Blade.
        pub static mut METAL_BLADE_STATUS_PATCH_1 : usize = 0x1080284;
        pub static mut METAL_BLADE_STATUS_PATCH_2 : usize = 0x1080288;
    }

    /// Ryu and Ken share the same fighter code.
    pub mod shotos {
        pub static mut INIT : usize = 0x10d4570;
        pub static mut WHAT_IS_THIS : usize = 0x646fe0;
        pub static mut WHAT_IS_THIS_2 : usize = 0x6da350;
        pub static mut MOVE_STRENGTH_AUTOTURN_HANDLER : usize = 0x10d4df0;
        pub static mut HANDLE_LIGHT_NORMALS : usize = 0x10d5a80;
        pub static mut ON_SITUATION_CHANGE : usize = 0x10d6c10;
        pub static mut ON_HIT : usize = 0x10d6ca0;
        pub static mut ON_HIT_2 : usize = 0x10d7420;
        pub static mut ON_SEARCH : usize = 0x10d7b80;
        pub static mut ON_DAMAGE : usize = 0x10d7760;
        pub static mut FRAME : usize = 0x10d5f30;
        pub static mut STATUS_CHANGE_CALLBACK : usize = 0x10d6a10;
        /// Patches the original final smash cancel check.
        pub static mut FINAL_SMASH_CANCEL_CHECK_PATCH : usize = 0x10d6324;
        /// Allows any status over 0x1de to be final smash cancelable.
        pub static mut FINAL_SMASH_CANCEL_STATUS_PATCH : usize = 0x10d67d8;
        /// Some kind of transition check table for Ryu (byte at +0x1F8 is patched).
        pub static mut RYU_TRANSITION_CHECK_TABLE : usize = 0x5032eb0;
    }

    pub mod shulk {
        pub static mut CHECK_VALID_ARTS_STATUSES : usize = 0x116a3d0;
        pub static mut CHECK_CAN_ACTIVATE_ART_WHEEL : usize = 0x116d8a0;
        pub static mut CAN_TRANSITION_TO_SPECIAL_N : usize = 0x116b7f0;
        pub static mut INC_ARTS_WHEEL_BUTTON_TIMER : usize = 0x1168360;
        pub static mut ON_ATTACK : usize = 0x116dbb0;
        /// Disables a weird check that forces you to go into wait/fall while holding Special, probably for the Art Wheel.
        pub static mut SPECIAL_HOLD_WAIT_FALL_PATCH : usize = 0x1167170;
        /// Initial checks that put Shulk into the art wheel animation.
        pub static mut ART_WHEEL_ANIMATION_PATCH_1 : usize = 0x11684dc;
        pub static mut ART_WHEEL_ANIMATION_PATCH_2 : usize = 0x11684f0;
        /// Multiplying the params for art cooldown by 60.
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_1 : usize = 0x116a680;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_2 : usize = 0x116a6c4;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_3 : usize = 0x116a708;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_4 : usize = 0x116a74c;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_5 : usize = 0x116a790;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_6 : usize = 0x116b948;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_7 : usize = 0x116b98c;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_8 : usize = 0x116b9d0;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_9 : usize = 0x116ba14;
        pub static mut ART_COOLDOWN_MULTIPLY_PATCH_10 : usize = 0x116ba58;
        /// Same as above, for the meter handler.
        pub static mut ART_COOLDOWN_METER_HANDLER_PATCH_1 : usize = 0x1169a1c;
        pub static mut ART_COOLDOWN_METER_HANDLER_PATCH_2 : usize = 0x1169ce4;
        /// The timer decrease for art cooldowns.
        pub static mut ART_COOLDOWN_TIMER_DECREASE_PATCH : usize = 0x1169770;
    }

    pub mod sonic {
        pub static mut ON_HIT : usize = 0x11d5a00;

        pub mod vtable {
            pub static mut ON_INIT : usize = 0x5046570;
        }
    }

    pub mod wario {
        pub static mut VTABLE_FUNC : usize = 0x1286ae0;
        pub static mut VTABLE_FUNC_2 : usize = 0x128b0b0;
    }
}

pub mod weapon {
    /// Shared weapon code.
    pub mod common {
        pub static mut INIT : usize = 0x33a07d0 + 0x5b0;
        /// Used for when generic weapons hit something else.
        pub static mut ATTACK_CALLBACK : usize = 0x33a8010 + 0x5b0;
        pub static mut HIT_HANDLER_2 : usize = 0x33bcd10 + 0x5b0;
    }

    pub mod belmont_cross {
        pub static mut ON_HIT : usize = 0x34f8280 + 0x5b0;
        pub static mut ON_HIT_THING : usize = 0x33a9bf0 + 0x5b0;
    }

    pub mod blaster_bullet {
        pub static mut GENERATE_ANGLE : usize = 0xa633f4;
        /// Patches offsetting the ecb.
        pub static mut ECB_OFFSET_PATCH : usize = 0x33faaec + 0x5b0;
    }

    pub mod dolly_burst {
        pub static mut CHECK_STATUS : usize = 0x97569c;
        pub static mut SET_MOTION : usize = 0x975b70;
        pub static mut INIT : usize = 0x33df440 + 0x5b0;
        pub static mut ON_HIT : usize = 0x33df620 + 0x5b0;
    }

    pub mod dolly_wave {
        pub static mut INIT : usize = 0x33e1800 + 0x5b0;
        pub static mut ON_HIT : usize = 0x33e1d34 + 0x5b0;

        pub mod vtable {
            pub static mut ON_HIT_2 : usize = 0x51bdd10;
        }
    }

    pub mod edge_flare1 {
        pub static mut INIT : usize = 0x33eccf0 + 0x5b0;
    }

    pub mod ike_sword {
        pub static mut SET_SPAWN_POS : usize = 0xaf9bf0;
        pub static mut SET_STATUS : usize = 0xaf9cc4;
        pub static mut ON_HIT : usize = 0x340ac50 + 0x5b0;

        pub mod vtable {
            pub static mut CAN_POCKET : usize = 0x51ce2d0;
        }
    }

    pub mod inkling_rollerink {
        pub static mut GENERATE : usize = 0xb103bc;
    }

    pub mod koopa_breath {
        pub static mut ON_HIT : usize = 0x34216e0 + 0x5b0;
    }

    pub mod mariod_drcapsule {
        /// Pill fix for respawn platform.
        pub static mut RESPAWN_PLATFORM_PILL_PATCH : usize = 0xcc9e34;
    }

    pub mod packun_poisonbreath {
        pub mod vtable {
            pub static mut ON_HIT : usize = 0x51f6f90;
        }
    }

    pub mod ryu_shinkuhadoken {
        pub mod vtable {
            pub static mut ON_HIT : usize = 0x5215940;
        }
    }
}

/// Offsets relative to the base of the `item` NRO. They are resolved against
/// the module base when the NRO is loaded.
pub mod item {
    pub mod holywater {
        pub static mut SIMON_THROW : usize = 0x792300;
        pub static mut RICHTER_THROW : usize = 0x757e20;
        pub static mut RICHTER_BORN : usize = 0x758e00;
        pub static mut RICHTER_BORN_LOOP : usize = 0x759600;
    }

    pub mod linkarrow {
        pub static mut THROW : usize = 0x6ca62c;
    }

    pub mod snake_grenade {
        pub static mut STATUS_FALL : usize = 0x7c9ae0;
        pub static mut STATUS_LANDING : usize = 0x7c9d10;
        pub static mut STATUS_THROWN : usize = 0x7c9fc0;
        /// Makes it so hitting the grenade launches it instead of exploding.
        pub static mut HIT_LAUNCH_PATCH : usize = 0x7ca48c;
    }
}

pub mod system {
    /// BattleObjectModuleAccessor lifecycle functions hooked for VarModule setup.
    pub mod module_accessor {
        pub static mut INITIALIZE_MODULES : usize = 0x3af300;
        pub static mut START_MODULES : usize = 0x3afa10;
        pub static mut END_MODULES : usize = 0x3afe00;
        pub static mut FINALIZE_MODULES : usize = 0x3af720;
    }

    pub mod command {
        pub static mut C_236 : usize = 0x6bef70;
        pub static mut C_41236 : usize = 0x6bf070;
        pub static mut C_214 : usize = 0x6bf3b0;
        pub static mut C_623 : usize = 0x6bf210;
        pub static mut C_632 : usize = 0x6bf630;
        pub static mut C_21416 : usize = 0x6bfcd0;
        pub static mut C_21416_R : usize = 0x6bff10;
        pub static mut C_236236 : usize = 0x6bf8d0;
        pub static mut C_236236_R : usize = 0x6bfae0;
        pub static mut C_623_NB : usize = 0x6c0120;
        pub static mut C_623_STRICT : usize = 0x6c0270;
        pub static mut C_623_AB_LONG : usize = 0x6c0480;
        pub static mut C_323_CATCH : usize = 0x6c0df0;
    }

    pub mod control_module {
        pub static mut EXEC_COMMAND : usize = 0x6bac10;
        pub static mut SET_ATTACK_AIR_STICK : usize = 0x6be630;
        pub static mut SET_HOLD_BUFFER_VALUE : usize = 0x6bd5b4;
        pub static mut SET_RELEASE_VALUE_IN_HITLAG : usize = 0x6bd51c;
        pub static mut SET_RELEASE_VALUE : usize = 0x6bd5d8;
        pub static mut CHECK_SKIP_HITLAG_BUFFER : usize = 0x6bd4a0;
        /// Prevents buffered C-stick aerials from triggering nair.
        pub static mut BUFFERED_CSTICK_NAIR_PATCH : usize = 0x6be664;
        /// Prevents Aerial Kind resetting every frame.
        pub static mut RESET_ATTACK_AIR_KIND_PATCH : usize = 0x6bd6c4;
        /// Removes 10f C-stick lockout for tilt stick and special stick.
        pub static mut CSTICK_LOCKOUT_PATCH_1 : usize = 0x17532ac - 0x1A0;
        pub static mut CSTICK_LOCKOUT_PATCH_2 : usize = 0x17532b0 - 0x1A0;
        pub static mut CSTICK_LOCKOUT_PATCH_3 : usize = 0x17532b4 - 0x1A0;
        pub static mut CSTICK_LOCKOUT_PATCH_4 : usize = 0x17532b8 - 0x1A0;
        /// Always uses the hitlag handling that cat4 uses.
        pub static mut HITLAG_BUFFER_CAT4_PATCH : usize = 0x6bd448;
        /// Stubs the check if the buffer value is 1 and the button is held.
        pub static mut HOLD_BUFFER_CHECK_PATCH : usize = 0x6bd5b0;
        /// Stubs setting the buffer lifetime to 2 if held.
        pub static mut HOLD_BUFFER_LIFETIME_PATCH : usize = 0x6bd53c;
    }

    pub mod grab_module {
        /// Disables the LR check.
        pub static mut LR_CHECK_PATCH : usize = 0x45c85c;
    }

    pub mod controller {
        pub static mut PACKED_PACKET_CREATION : usize = 0x16d85dc - 0x1A0;
        pub static mut WRITE_PACKET : usize = 0x16d8610 - 0x1A0;
        pub static mut MAP_CONTROLS : usize = 0x1750f70 - 0x1A0;
        pub static mut HANDLE_INCOMING_PACKET : usize = 0x16d7034 - 0x1A0;
        pub static mut PARSE_INPUTS : usize = 0x3f7240;
        pub static mut AFTER_EXEC : usize = 0x6b9c7c;
        /// Pointer to the table used by `get_mapped_controller_inputs_from_id`.
        pub static mut MAPPED_INPUTS_TABLE : usize = 0x52c50f0;
        pub static mut POST_GAMECUBE_PROCESS : usize = 0x3666eac + 0x5B0;
        pub static mut APPLY_TRIGGERS : usize = 0x3666d0c + 0x5B0;
        pub static mut ANALOG_TRIGGER_L : usize = 0x3666ee0 + 0x5B0;
        pub static mut ANALOG_TRIGGER_R : usize = 0x3666ef4 + 0x5B0;
        pub static mut ANALOG_TRIGGER_L_PATCH : usize = 0x3666edc + 0x5B0;
        pub static mut ANALOG_TRIGGER_R_PATCH : usize = 0x3666ef0 + 0x5B0;
    }

    pub mod css {
        pub static mut FIX_CHARA_REPLACE : usize = 0x1798ac8 - 0xE0;
    }

    pub mod menu {
        pub static mut MAIN_MENU_QUICK : usize = 0x235cad0 + 0x450;
    }

    pub mod music {
        pub static mut MUSIC_FUNCTION_1 : usize = 0x23ed810 + 0x450;
        pub static mut MUSIC_FUNCTION_2 : usize = 0x23ee0c0 + 0x450;
        pub static mut TRAINING_RESET_MUSIC_1 : usize = 0x1509fd4;
        pub static mut TRAINING_RESET_MUSIC_2 : usize = 0x14f99cc;
    }

    pub mod fighterutil {
        pub static mut GET_MOTION_DATA : usize = 0x6e2440;
        /// Removes the Diddy fighter kind check from the diddy unlink node FighterUtil function.
        pub static mut DIDDY_UNLINK_NODE_KIND_CHECK_PATCH : usize = 0x6938b4;
    }

    pub mod energy {
        pub static mut ADJUST_SPEED_FOR_GROUND_NORMAL : usize = 0x47b4f0;
        pub static mut PROCESS : usize = 0x47bf90;

        pub mod control {
            pub static mut UPDATE : usize = 0x6d3630;
            pub static mut INITIALIZE : usize = 0x6d4060;
            pub static mut SETUP : usize = 0x6d4bc0;
            /// ControlModule::get_stick_x calls when calculating horizontal jump velocity.
            pub static mut JUMP1_STICK_X : usize = 0x6ce6d8;
            pub static mut JUMP2_STICK_X : usize = 0x6d19c4;
            pub static mut JUMP3_STICK_X : usize = 0x6d1b10;
            pub static mut JUMP4_STICK_X : usize = 0x6d0454;
            /// Same as above but for double jumps.
            pub static mut JUMP_AERIAL_STICK_X : usize = 0x6ce7d0;
            pub static mut JUMP_AERIAL_2_STICK_X : usize = 0x6d05cc;
            pub static mut JUMP_AERIAL_3_STICK_X : usize = 0x6d117c;
            pub static mut JUMP_AERIAL_4_STICK_X : usize = 0x6ce28c;
            /// Super Jump Speed Multiplier.
            pub static mut JUMP_SPEED_Y : usize = 0x6d253c;
            /// Always use Jump Speed Y.
            pub static mut ALWAYS_USE_JUMP_SPEED_Y_PATCH : usize = 0x6d217c;
        }

        pub mod damage {
            pub static mut INITIALIZE : usize = 0x6d8100;
        }
    }

    pub mod engine {
        pub static mut CHANGE_ELEC_HITLAG_FOR_ATTACKER : usize = 0x406824;
        /// Also nopped to stub the parry hitlag calculation.
        pub static mut SET_PARRY_HITLAG : usize = 0x641d84;
        /// Also nopped to remove the vanilla ledge trump check.
        pub static mut REVERSE_TRUMP_LOGIC : usize = 0x617aa4;
        pub static mut FORCE_REFLECT_FULL_LIFETIME : usize = 0x33bdd88 + 0x5b0;
        pub static mut SHIELD_BREAK_LR_SET : usize = 0x6416e8;
        pub static mut SHIELD_SET_FACING_LR : usize = 0x6418f8;
        pub static mut SHIELD_HEALTH_RECOVERY_CHECK_MAX : usize = 0x614c0c;
        /// Also nopped as part of the shield health recovery patch.
        pub static mut SHIELD_HEALTH_RECOVERY_CHECK_LESS_THAN_MAX : usize = 0x614b9c;
        pub static mut FIGHTER_GLOBAL_PER_FRAME : usize = 0x614630;
        pub static mut DAMAGE_LEVEL : usize = 0x403ca4;
        pub static mut HITSTUN_GRAVITY_1 : usize = 0x6d249c;
        pub static mut HITSTUN_GRAVITY_2 : usize = 0x6c39a0;
        pub static mut HITSTUN_GRAVITY_3 : usize = 0x6d5924;
        pub static mut HITSTUN_FALL_SPEED_1 : usize = 0x6d24c4;
        pub static mut HITSTUN_FALL_SPEED_2 : usize = 0x6c39c8;
        pub static mut HITSTUN_FALL_SPEED_3 : usize = 0x6d594c;
        pub static mut FORCE_NORMAL_HITSTUN_FALLSPEEDS_1 : usize = 0x6d1654;
        pub static mut FORCE_NORMAL_HITSTUN_FALLSPEEDS_2 : usize = 0x6c35f8;
        /// Removes Phantom Hits.
        pub static mut PHANTOM_HIT_PATCH : usize = 0x3e6d08;
        /// Removes the vanilla ledge trump check (together with `REVERSE_TRUMP_LOGIC`).
        pub static mut LEDGE_TRUMP_CHECK_PATCH : usize = 0x617a90;
        /// Removes the forced change to HIT_STATUS_OFF during Final Smash.
        pub static mut FINAL_SMASH_HIT_STATUS_OFF_PATCH : usize = 0x62d5ac;
        /// Disables the reversed stick values when autoturn runs.
        pub static mut AUTOTURN_REVERSE_STICK_PATCH_1 : usize = 0x69ae20;
        pub static mut AUTOTURN_REVERSE_STICK_PATCH_2 : usize = 0x934a6c;
        pub static mut AUTOTURN_REVERSE_STICK_PATCH_3 : usize = 0x974d20;
        pub static mut AUTOTURN_REVERSE_STICK_PATCH_4 : usize = 0x21d7d1c;
        /// Disables Reverse Special Command calls.
        pub static mut REVERSE_SPECIAL_COMMAND_PATCH_1 : usize = 0x69ad9c;
        pub static mut REVERSE_SPECIAL_COMMAND_PATCH_2 : usize = 0x974d00;
        pub static mut REVERSE_SPECIAL_COMMAND_PATCH_3 : usize = 0x934a4c;
        pub static mut REVERSE_SPECIAL_COMMAND_PATCH_4 : usize = 0x21d7cfc;
        /// Removes the 3f delay on backdashing for Ryu/Ken/Terry/Kazuya.
        pub static mut BACKDASH_DELAY_PATCH : usize = 0x69aef8;
        /// Removes the ledge grab limit.
        pub static mut LEDGE_GRAB_LIMIT_PATCH_1 : usize = 0x618cc8;
        pub static mut LEDGE_GRAB_LIMIT_PATCH_2 : usize = 0x62f0b4;
        pub static mut LEDGE_GRAB_LIMIT_PATCH_3 : usize = 0x62f0b8;
        /// Use fall speed for vertical launchers (acceleration).
        pub static mut VERTICAL_LAUNCH_ACCEL_1 : usize = 0x6c3988;
        pub static mut VERTICAL_LAUNCH_ACCEL_2 : usize = 0x6d2480;
        pub static mut VERTICAL_LAUNCH_ACCEL_3 : usize = 0x6d590c;
        /// Use fall speed for vertical launchers (speed).
        pub static mut VERTICAL_LAUNCH_SPEED_1 : usize = 0x6c39b0;
        pub static mut VERTICAL_LAUNCH_SPEED_2 : usize = 0x6d24ac;
        pub static mut VERTICAL_LAUNCH_SPEED_3 : usize = 0x6d5934;
        /// Patches shield health recovery (together with `SHIELD_HEALTH_RECOVERY_CHECK_LESS_THAN_MAX`).
        pub static mut SHIELD_HEALTH_RECOVERY_PATCH : usize = 0x614ba0;
        /// Changes the >= check to > when reducing shield health to exactly 0.0.
        pub static mut SHIELD_HEALTH_ZERO_CHECK_PATCH : usize = 0x64160c;
        /// Disables getting airdodge back on hit.
        pub static mut AIRDODGE_ON_HIT_PATCH : usize = 0x632530;
        /// Allow 0 CPUs in Training Mode menu: allow UI to decrement to 0.
        pub static mut TRAINING_CPU_COUNT_UI_DECREMENT_PATCH : usize = 0x1bb46a4;
        /// Allow 0 CPUs in Training Mode menu: change set-value handler clamp to 0.
        pub static mut TRAINING_CPU_COUNT_SET_VALUE_CLAMP_PATCH : usize = 0x1bbad14;
        /// Allow 0 CPUs in Training Mode menu: fix clamp logic to clamp underflow to 0 instead of 1.
        pub static mut TRAINING_CPU_COUNT_CLAMP_UNDERFLOW_PATCH : usize = 0x1bbad18;
    }
}
