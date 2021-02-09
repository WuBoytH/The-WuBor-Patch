use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK,
    animation = "special_lw",
    animcmd = "game_speciallw")]
pub fn jack_dspecial(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=4)
        if(is_excute){
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
        frame(Frame=32)
        if(is_excute){
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK,
    animation = "special_air_lw",
    animcmd = "game_specialairlw")]
pub fn jack_dspecialair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=4)
        if(is_excute){
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
        frame(Frame=32)
        if(is_excute){
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK,
    animation = "special_lw_counter",
    animcmd = "game_speciallwcounter")]
pub fn jack_dspecialcounter(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=51, FKB=0, BKB=80, Size=11.0, X=0.0, Y=10.5, Z=-5.0, X2=0.0, Y2=10.5, Z2=10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
            AttackModule::set_force_reaction(0, true, false)
            AttackModule::set_force_reaction(1, true, false)
        }
        frame(Frame=8)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK,
    animation = "special_air_lw_counter",
    animcmd = "game_specialairlwcounter")]
pub fn jack_dspecialcounterair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=51, FKB=0, BKB=80, Size=11.0, X=0.0, Y=10.5, Z=-5.0, X2=0.0, Y2=10.5, Z2=10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
            AttackModule::set_force_reaction(0, true, false)
            AttackModule::set_force_reaction(1, true, false)
        }
        frame(Frame=8)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        jack_dspecial,
        jack_dspecialair,
        jack_dspecialcounter,
        jack_dspecialcounterair
    );
}