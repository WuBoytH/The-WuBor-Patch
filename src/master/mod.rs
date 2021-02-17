use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use acmd::{acmd, acmd_func};
use smash::app::lua_bind::*;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER,
    animation = "attack_s3",
    animcmd = "game_attack_s3")]
pub fn master_ftilt(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=4)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=2.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=3.5, X=4.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=3.5, X=8.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=3.5, X=12.5, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=76, FKB=0, BKB=48, Size=6.0, X=0.0, Y=7.5, Z=12.0, X2=0.0, Y2=7.5, Z2=18.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=1, false)
            AttackModule::clear(ID=2, false)
            AttackModule::clear(ID=3, false)
            AttackModule::clear(ID=4, false)
        }
        frame(Frame=13)
        if(is_excute){
            AttackModule::clear_all()
            FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 4.0)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER,
    animation = "attack_lw3",
    animcmd = "game_attack_lw3")]
pub fn master_dtilt(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
            ArticleModule::change_motion(*FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::phx::Hash40{hash: hash40("attack_lw3")}, false, 0.0)
        }
        frame(Frame=3)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=9)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=93, KBG=55, FKB=0, BKB=67, Size=2.8, X=0.0, Y=2.8, Z=10.0, X2=0.0, Y2=2.8, Z2=12.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=96, KBG=55, FKB=0, BKB=67, Size=2.8, X=0.0, Y=2.8, Z=17.0, X2=0.0, Y2=2.8, Z2=25.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=16)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=59)
        if(is_excute){
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_MASTER_SWORD, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn master_dtiltsword(fighter: &mut L2CFighterBase) {
    acmd!({
        if(is_excute){
            WorkModule::set_float(6.0, WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLOAT_2ND_GRAVITY)
            WorkModule::set_float(0.0, WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLOAT_2ND_AIR_RESISTANCE)
        }
        frame(Frame=3)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=9)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=24)
        if(is_excute){
            WorkModule::on_flag(Flag=WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLAG_PHYSICS)
        }
        frame(Frame=34)
        if(is_excute){
            WorkModule::off_flag(Flag=WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLAG_PHYSICS)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER,
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn master_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
        }
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=3.0, X=-0.5, Y=4.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=2.4, X=-0.5, Y=9.2, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=2.4, X=-0.5, Y=13.5, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=3.4, X=-0.5, Y=19.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=3.0, X=-0.5, Y=25.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=13)
        if(is_excute){
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=2.0, X=0.0, Y=5.0, Z=5.0, X2=0.0, Y2=5.0, Z2=13.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=2.5, X=0.0, Y=5.0, Z=16.5, X2=0.0, Y2=5.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=14)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=53)
        if(is_excute){
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER,
    animation = "special_s_start",
    animcmd = "game_specialsstart")]
pub fn master_sspecialstart(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
        }
        FT_MOTION_RATE(FSM=0.6)
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER,
    animation = "special_air_s_start",
    animcmd = "game_specialairsstart")]
pub fn master_sspecialstartair(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0)
        }
        FT_MOTION_RATE(FSM=0.6)
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER,
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn master_uspecial(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.3)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(2.0, 6.0)
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
            ArticleModule::change_motion(*FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::phx::Hash40{hash: hash40("special_hi")}, false, 0.0)
        }
        frame(Frame=8)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TURN_CHECK)
        }
        frame(Frame=11)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=368, KBG=100, FKB=30, BKB=0, Size=6.5, X=0.0, Y=8.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            rust {
                let vector = smash::phx::Vector2f{x: 18.0, y: 44.0};
                let top_hash = smash::phx::Hash40::new("top");
                AttackModule::set_vec_target_pos(module_accessor, 0, top_hash, &vector, 7, false);
            }
            AttackModule::set_no_finish_camera_ex(0, true, false)
            AttackModule::set_no_dead_all(true, false)
        }
        frame(Frame=17)
        if(is_excute){
            ArticleModule::change_status(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, WEAPON_MASTER_SWORD_STATUS_KIND_EXTEND, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=0.01, Angle=368, KBG=100, FKB=30, BKB=0, Size=5.5, X=2.0, Y=-1.0, Z=-1.0, X2=2.0, Y2=-1.0, Z2=-1.0, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("throw"), Damage=3.0, Angle=75, KBG=100, FKB=0, BKB=40, Size=4.5, X=2.0, Y=-1.0, Z=-1.0, X2=2.0, Y2=-1.0, Z2=-1.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            rust {
                let vector = smash::phx::Vector2f{x: 16.0, y: 40.0};
                let top_hash = smash::phx::Hash40::new("top");
                AttackModule::set_vec_target_pos(module_accessor, 0, top_hash, &vector, 4, false);
            }
            AttackModule::set_no_finish_camera_ex(0, true, false)
            AttackModule::set_no_dead_all(true, false)
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=0.01, Angle=368, KBG=100, FKB=30, BKB=0, Size=5.5, X=0.0, Y=2.0, Z=0.75, X2=0.0, Y2=-4.0, Z2=-1.5, Hitlag=0.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=true, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA_d, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("throw"), Damage=3.0, Angle=75, KBG=100, FKB=0, BKB=40, Size=2.0, X=0.0, Y=2.0, Z=0.75, X2=0.0, Y2=-4.0, Z2=-1.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            rust {
                let vector = smash::phx::Vector2f{x: 16.0, y: 41.0};
                let top_hash = smash::phx::Hash40::new("top");
                AttackModule::set_vec_target_pos(module_accessor, 0, top_hash, &vector, 2, false);
            }
            AttackModule::set_no_dead_all(true, false)
            AttackModule::set_no_finish_camera_ex(0, true, false)
        }
        frame(Frame=20)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_ENABLE_CATCH)
            ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=3.0, Angle=75, KBG=100, FKB=0, BKB=40, Size=5.5, X=0.0, Y=2.0, Z=0.75, X2=0.0, Y2=-4.0, Z2=-1.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_BODY, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("throw"), Damage=3.0, Angle=75, KBG=100, FKB=0, BKB=40, Size=2.0, X=0.0, Y=2.0, Z=0.75, X2=0.0, Y2=-4.0, Z2=-1.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_no_dead_all(true, false)
            AttackModule::set_no_finish_camera_ex(0, true, false)
        }
        frame(Frame=21)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=3.0, Angle=75, KBG=100, FKB=0, BKB=40, Size=5.5, X=0.0, Y=5.5, Z=2.0, X2=0.0, Y2=-6.0, Z2=-2.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, CollisionPart=COLLISION_PART_MASK_BODY, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("throw"), Damage=3.0, Angle=75, KBG=100, FKB=0, BKB=40, Size=2.0, X=0.0, Y=8.0, Z=3.0, X2=0.0, Y2=-4.0, Z2=-1.5, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_no_dead_all(true, false)
            AttackModule::set_no_finish_camera_ex(0, true, false)
        }
        frame(Frame=25)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=27)
        if(is_excute){
            ArticleModule::change_status(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, WEAPON_MASTER_SWORD_STATUS_KIND_BACK, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_MASTER_SWORD, 
    animation = "special_hi",
    animcmd = "game_special_hi")]
pub fn master_uspecialsword(fighter: &mut L2CFighterBase) {
    acmd!({
        FT_MOTION_RATE(FSM=0.3)
        if(is_excute){
            WorkModule::set_float(8.0, WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLOAT_2ND_GRAVITY)
            WorkModule::set_float(0.0, WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLOAT_2ND_AIR_RESISTANCE)
        }
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLAG_PHYSICS)
        }
        frame(Frame=10)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=11)
        if(is_excute){
            WorkModule::off_flag(Flag=WEAPON_MASTER_SWORD_INSTANCE_WORK_ID_FLAG_PHYSICS)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER,
    animation = "special_lw",
    animcmd = "game_speciallw")]
pub fn master_dspecial(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR)
        }
        frame(Frame=42)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK)
        }
        frame(Frame=51)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1)
        }
        frame(Frame=60)
        if(is_excute){
            AttackModule::set_attack_height(0, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
        }
        frame(Frame=64)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR)
            ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
        }
        frame(Frame=65)
        if(is_excute){
            ArticleModule::set_flag(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND)
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
        }
        frame(Frame=96)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY)
            WorkModule::on_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED)
        }
        frame(Frame=117)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2)
        }
        frame(Frame=118)
        if(is_excute){
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }            
    });
}

pub fn install() {
    acmd::add_hooks!(
        master_ftilt,
        master_dtilt,
        master_dtiltsword,
        master_fair,
        master_sspecialstart,
        master_sspecialstartair,
        master_uspecial,
        master_uspecialsword,
        master_dspecial
    );
}