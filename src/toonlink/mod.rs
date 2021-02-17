use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
//use smash::phx::Vector3f;
//use smash::app::BattleObjectModuleAccessor;
//use smash::app::lua_bind::EffectModule;

pub fn special_hi_move(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_TOONLINK {
            if MotionModule::motion_kind(module_accessor) == smash::hash40("special_hi") {
                if MotionModule::frame(module_accessor) > 6.0 && MotionModule::frame(module_accessor) < 46.0 {
                    let facing_dirn = PostureModule::lr(module_accessor);
                    if facing_dirn > 0.0 {
                        acmd!(lua_state, {
                            SET_SPEED_EX(1.76 * ControlModule::get_stick_x(module_accessor), 0.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
                        });
                    }
                    else{
                        acmd!(lua_state, {
                            SET_SPEED_EX(-1.76 * ControlModule::get_stick_x(module_accessor), 0.5, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
                        });
                    }
                }
            }
        }
    }
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_TOONLINK,
    animation = "attack_dash",
    animcmd = "game_attack_dash")]
pub fn toonlink_dashattack(fighter: &mut L2CFighterCommon) {
    acmd!({
        FT_MOTION_RATE(FSM=0.7)
        frame(Frame=8)
        FT_MOTION_RATE(FSM=1.1)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=82, KBG=70, FKB=0, BKB=60, Size=4.2, X=5.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=82, KBG=70, FKB=0, BKB=60, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=6.0, Angle=82, KBG=70, FKB=0, BKB=60, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        FT_MOTION_RATE(FSM=0.6186)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=43)
        FT_MOTION_RATE(FSM=1)
    });
}


pub fn install() {
    acmd::add_hooks!(
        toonlink_dashattack
    );
    acmd::add_custom_hooks!(
        special_hi_move
    );
}