use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use smash::phx::Vector3f;
use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::EffectModule;

static mut SPECIAL_LW : [bool; 8] = [false; 8];
static mut SPECIAL_LW_TIMER : [i16; 8] = [-1; 8];

pub unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    return entry_id;
}

pub fn reset_vars(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            SPECIAL_LW[entry_id(module_accessor)] = false;
            SPECIAL_LW_TIMER[entry_id(module_accessor)] = -1;
            println!("Reset all vars!");
        }
        else {
        }
    }
}

pub fn special_lw_check(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_RYU {
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && ControlModule::get_stick_y(module_accessor) < -0.5 {
                println!("Focus!");
                if MotionModule::motion_kind(module_accessor) == smash::hash40("special_n") {
                    println!("Hadoken!");
                    if MotionModule::frame(module_accessor) > 7.0 && SPECIAL_LW[entry_id(module_accessor)] == false {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                        SPECIAL_LW_TIMER[entry_id(module_accessor)] = 600;
                        SPECIAL_LW[entry_id(module_accessor)] = true;
                    }
                }
                if MotionModule::motion_kind(module_accessor) == smash::hash40("special_s_start") {
                    println!("Tatsu!");
                    if MotionModule::frame(module_accessor) > 8.0 && SPECIAL_LW[entry_id(module_accessor)] == false {
                        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                            SPECIAL_LW_TIMER[entry_id(module_accessor)] = 600;
                            SPECIAL_LW[entry_id(module_accessor)] = true;
                        }
                    }
                }
                if MotionModule::motion_kind(module_accessor) == smash::hash40("special_s") {
                    println!("Tatsu!");
                    if SPECIAL_LW[entry_id(module_accessor)] == false {
                        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                            SPECIAL_LW_TIMER[entry_id(module_accessor)] = 600;
                            SPECIAL_LW[entry_id(module_accessor)] = true;
                        }
                    }
                }
                if MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi_command") {
                    println!("Shoryu!");
                    if MotionModule::frame(module_accessor) > 5.0 && SPECIAL_LW[entry_id(module_accessor)] == false && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
                        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                            SPECIAL_LW_TIMER[entry_id(module_accessor)] = 600;
                            SPECIAL_LW[entry_id(module_accessor)] = true;
                        }
                    }
                }
            }
            if SPECIAL_LW_TIMER[entry_id(module_accessor)] > 0 {
                SPECIAL_LW_TIMER[entry_id(module_accessor)] = SPECIAL_LW_TIMER[entry_id(module_accessor)] - 1;
                println!("Timer decreased to {}", SPECIAL_LW_TIMER[entry_id(module_accessor)]);
            }
            else if SPECIAL_LW_TIMER[entry_id(module_accessor)] == 0 {
                SPECIAL_LW_TIMER[entry_id(module_accessor)] = -1;
                let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                let focuseff: u32 = EffectModule::req_follow(module_accessor, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                EffectModule::set_rgb(module_accessor, focuseff, 0.0, 2.0, 5.0);
                println!("SPECIAL_LW set to false!");
            }
            else{
                SPECIAL_LW[entry_id(module_accessor)] = false;
            }
        }
    }
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RYU,
    animation = "attack_hi3_s",
    animcmd = "game_attackhi3s")]
pub fn ryu_utilts(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
            WorkModule::on_flag(Flag=FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
        }
        frame(Frame=4)
        if(is_excute){
            HIT_NODE(hash40("head"), HIT_STATUS_XLU)
            HIT_NODE(hash40("bust"), HIT_STATUS_XLU)
            HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
            HIT_NODE(hash40("shoulderr"), HIT_STATUS_XLU)
            HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
            HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        }
        if (MotionModule::frame(module_accessor) > 7.0) {
            if (AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)) {
                if (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP)) {
                  StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=12.0, Angle=80, KBG=70, FKB=0, BKB=70, Size=3.0, X=1.7, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_RYU_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=12.0, Angle=80, KBG=70, FKB=0, BKB=70, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_RYU_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=12.0, Angle=80, KBG=70, FKB=0, BKB=70, Size=5.0, X=2.3, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_RYU_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=70, FKB=0, BKB=70, Size=2.5, X=0.0, Y=9.0, Z=12.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_RYU_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear(ID=3, false)
        }
        wait(Frames=3)
        if(is_excute){
            HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::off_flag(Flag=FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        }
        frame(Frame=21)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        ryu_utilts
    );
    acmd::add_custom_hooks!(
        reset_vars,
        special_lw_check
    );
}