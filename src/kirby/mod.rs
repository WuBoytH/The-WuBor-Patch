use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use smash::phx::Hash40;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KIRBY,
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn kirby_uair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.375)
        frame(Frame=9)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=10)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=10.0, Angle=70, KBG=115, FKB=0, BKB=20, Size=4.0, X=0.0, Y=-5.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=10.0, Angle=70, KBG=115, FKB=0, BKB=20, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=6)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=6)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KIRBY,
    animation = "special_s_start",
    animcmd = "game_specialsstart")]
pub fn kirby_sspecialstart(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0)
        }
        frame(Frames=3)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frames=15)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KIRBY,
    animation = "special_air_s_start",
    animcmd = "game_specialairsstart")]
pub fn kirby_sspecialstartair(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0)
        }
        frame(Frames=1)
        FT_MOTION_RATE(FSM=0.667)
        frame(Frames=25)
        FT_MOTION_RATE(FSM=1)
    });
}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KIRBY,
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn kirby_uspecial(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0)
            ArticleModule::change_motion(FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40{hash: hash40("special_hi")}, false, 0.0)
        }
        frame(Frames=5)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frames=19)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KIRBY,
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
pub fn kirby_uspecialair(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0)
            ArticleModule::change_motion(FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40{hash: hash40("special_hi")}, false, 0.0)
        }
        frame(Frames=5)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frames=19)
        FT_MOTION_RATE(FSM=1)
    });
}

pub fn install() {
    acmd::add_hooks!(
        kirby_uair,
        kirby_sspecialstart,
        kirby_sspecialstartair,
        kirby_uspecial,
        kirby_uspecialair
    );
}