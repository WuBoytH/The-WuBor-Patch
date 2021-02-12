use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::lua_bind::*;
use acmd::{acmd,acmd_func};
use smash::hash40;
use smash::phx::Vector3f;
use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::EffectModule;
use crate::custom::{TIME_SLOW_EFFECT_VECTOR, TIME_SLOW_EFFECT_HASH};

pub unsafe fn entry_id(module_accessor: &mut BattleObjectModuleAccessor) -> usize {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    return entry_id;
}

static mut SPECIAL_LW : [bool; 8] = [false; 8];
static mut SPECIAL_LW_TIMER : [i16; 8] = [-1; 8];
static mut DAMAGE_TAKEN : [f32; 8] = [0.0; 8];

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn shulk_is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    if fighter_kind == *FIGHTER_KIND_SHULK {
        if SPECIAL_LW[entry_id(module_accessor)] {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                return false;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
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

pub fn damage_check(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_SHULK {
            if StopModule::is_damage(module_accessor) {
                DAMAGE_TAKEN[entry_id(module_accessor)] = WorkModule::get_float(module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
                if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    if SPECIAL_LW[entry_id(module_accessor)] == false {
                        let launch_speed = KineticModule::get_sum_speed_x(module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
                        let facing_dirn = PostureModule::lr(module_accessor);
                        if launch_speed > 0.0 && facing_dirn > 0.0 {
                            PostureModule::reverse_lr(module_accessor);
                        }
                        else if launch_speed < 0.0 && facing_dirn < 0.0 {
                            PostureModule::reverse_lr(module_accessor);
                        }
                        else{

                        }
                        DamageModule::add_damage(module_accessor, DAMAGE_TAKEN[entry_id(module_accessor)] * -0.5, 0);
                        KineticModule::change_kinetic(module_accessor,*FIGHTER_KINETIC_TYPE_RESET);
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, true);
                        SPECIAL_LW[entry_id(module_accessor)] = true;
                        SPECIAL_LW_TIMER[entry_id(module_accessor)] = 3600;
                        println!("SPECIAL_LW set to true, timer set to 3600!");
                    }
                }
            }
        }
    }
}

pub fn special_lw_check(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_SHULK {
            if StatusModule::status_kind(module_accessor) == *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT {
                acmd!(lua_state, {
                    SLOW_OPPONENT(20, 60)
                });
                EffectModule::req_on_joint(module_accessor, smash::phx::Hash40::new_raw(TIME_SLOW_EFFECT_HASH), smash::phx::Hash40::new("head"), &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, 1.0, &TIME_SLOW_EFFECT_VECTOR, &TIME_SLOW_EFFECT_VECTOR, false, 0, 0, 0);
            }
            if SPECIAL_LW_TIMER[entry_id(module_accessor)] > 0 {
                SPECIAL_LW_TIMER[entry_id(module_accessor)] = SPECIAL_LW_TIMER[entry_id(module_accessor)] - 1;
                println!("Timer decreased to {}", SPECIAL_LW_TIMER[entry_id(module_accessor)]);
            }
            else if SPECIAL_LW_TIMER[entry_id(module_accessor)] == 0 {
                SPECIAL_LW_TIMER[entry_id(module_accessor)] = -1;
                let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                let countereff: u32 = EffectModule::req_follow(module_accessor, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                EffectModule::set_rgb(module_accessor, countereff, 0.0, 5.0, 5.0);
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
    battle_object_kind = FIGHTER_KIND_SHULK,
    animation = "attack_11",
    animcmd = "game_attack11")]
pub fn shulk_jab1(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=2.8, X=0.0, Y=9.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=35, Size=2.8, X=0.0, Y=9.0, Z=4.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=180, KBG=10, FKB=0, BKB=25, Size=3.0, X=0.0, Y=9.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_FIGHTER, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=361, KBG=10, FKB=0, BKB=25, Size=3.0, X=0.0, Y=9.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=2.0, Unk=false)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=10)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
        }
        frame(Frame=14)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        shulk_jab1
    );
    acmd::add_custom_hooks!(
        reset_vars,
        special_lw_check,
        damage_check
    );
    skyline::install_hook!(shulk_is_enable_transition_term_replace);
}
