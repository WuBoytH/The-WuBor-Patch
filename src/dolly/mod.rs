use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_DOLLY,
    animation = "special_lw_attack",
    animcmd = "game_specialairlw")]
pub fn dolly_dspecialair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=0)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.3, y: -1.0, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
                WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            }
            else{
                if(is_excute){
                    WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 1.3, y: -1.5, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                    WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
                }
            }
        }
        frame(Frame=1)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: -0.3, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: -1.0, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=2)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: -0.5, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=3)
        if(WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND)){
            if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=50, KBG=70, FKB=0, BKB=80, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_PUNCH)
                }
                else{
                    if(WorkModule::get_int(module_accessor, *ATTACK_REGION_PUNCH) == *FIGHTER_DOLLY_STRENGTH_W){
                        if(is_excute){
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=50, KBG=65, FKB=0, BKB=80, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_PUNCH)
                        }
                        else{
                            if(is_excute){
                                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=50, KBG=75, FKB=0, BKB=80, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_CRITICAL, Type=ATTACK_REGION_PUNCH)
                            }
                            else{
                                if(is_excute){
                                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=50, KBG=70, FKB=0, BKB=80, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_PUNCH)
                                }
                            }
                        }
                    }
                }
            }
        }
        frame(Frame=4)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=5)
        if(WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND)){
            if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=50, KBG=80, FKB=0, BKB=60, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_PUNCH)
                }
                else{
                    if(WorkModule::get_int(module_accessor, *ATTACK_REGION_PUNCH) == *FIGHTER_DOLLY_STRENGTH_W){
                        if(is_excute){
                            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=50, KBG=80, FKB=0, BKB=60, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_PUNCH)
                        }
                        else{
                            if(is_excute){
                                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=310, KBG=95, FKB=0, BKB=30, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_CRITICAL, Type=ATTACK_REGION_PUNCH)
                            }
                            else{
                                if(is_excute){
                                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=50, KBG=80, FKB=0, BKB=60, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_PUNCH)
                                }
                            }
                        }
                    }
                }
            }
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                MotionModule::set_rate(1.2)
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=6)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=7)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=8)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=9)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=10)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=11)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.05, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=12)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=13)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                rust{
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
                    KineticModule::add_speed(module_accessor, &speed_vector);
                }
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(1.0)
                    rust{
                        let speed_vector = smash::phx::Vector3f { x: 0.0, y: 0.2, z: 0.0 };
                        KineticModule::add_speed(module_accessor, &speed_vector);
                    }
                }
            }
        }
        frame(Frame=14)
        if(WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND)){
            if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=50, KBG=80, FKB=0, BKB=60, Size=7.0, X=0.0, Y=8.0, Z=5.5, X2=0.0, Y2=4.0, Z2=3.5, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_PUNCH)
                }
            }
        }
        frame(Frame=15)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                MotionModule::set_rate(10.0)
            }
            else{
                if(is_excute){
                    MotionModule::set_rate(10.0)
                }
            }
        }
        frame(Frame=35)
        if(is_excute){
            AttackModule::clear_all()
            KineticModule::change_kinetic(FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL)
            MotionModule::set_rate(1.0)
        }
        frame(Frame=37)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_DOLLY,
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
pub fn dolly_uspecialair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR)
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH)
        }
        frame(Frame=9)
        if(is_excute){
            HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
            HIT_NODE(hash40("kneel"), HIT_STATUS_XLU)
            HIT_NODE(hash40("legr"), HIT_STATUS_XLU)
            HIT_NODE(hash40("legl"), HIT_STATUS_XLU)
            HIT_NODE(hash40("hip"), HIT_STATUS_XLU)
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=98, KBG=110, FKB=110, BKB=30, Size=4.0, X=0.0, Y=10.0, Z=6.5, X2=0.0, Y2=10.0, Z2=2.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=75, KBG=110, FKB=110, BKB=30, Size=4.0, X=0.0, Y=10.0, Z=-2.0, X2=0.0, Y2=10.0, Z2=-3.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            AttackModule::set_no_finish_camera(0, true, false)
            AttackModule::set_no_finish_camera(1, true, false)
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        }
        frame(Frame=11)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=4.5, Z=0.0, X2=0.0, Y2=4.5, Z2=6.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=30, FKB=30, BKB=30, Size=3.5, X=0.0, Y=17.0, Z=1.0, X2=0.0, Y2=19.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=10, Size=4.5, X=0.0, Y=10.0, Z=3.0, X2=0.0, Y2=10.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=45, FKB=40, BKB=10, Size=4.5, X=0.0, Y=10.0, Z=3.0, X2=0.0, Y2=10.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=4.5, Z=-4.0, X2=0.0, Y2=4.5, Z2=-6.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::off_flag(Flag=FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        }
        frame(Frames=13)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=3.5, Z=6.0, X2=0.0, Y2=3.5, Z2=0.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=30, FKB=30, BKB=30, Size=3.5, X=0.0, Y=16.0, Z=1.0, X2=0.0, Y2=18.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=10, Size=4.5, X=0.0, Y=9.0, Z=3.0, X2=0.0, Y2=9.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=40, FKB=40, BKB=10, Size=4.5, X=0.0, Y=9.0, Z=3.0, X2=0.0, Y2=9.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=3.5, Z=-4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=3.5, Z=0.0, X2=0.0, Y2=3.5, Z2=6.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=30, FKB=30, BKB=30, Size=3.5, X=0.0, Y=16.0, Z=1.0, X2=0.0, Y2=18.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=10, Size=4.5, X=0.0, Y=9.0, Z=3.0, X2=0.0, Y2=9.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=40, FKB=40, BKB=10, Size=4.5, X=0.0, Y=9.0, Z=3.0, X2=0.0, Y2=9.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=3.5, Z=-4.0, X2=0.0, Y2=3.5, Z2=-6.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("hip"), HIT_STATUS_NORMAL)
            AttackModule::clear_all()
        }
        frame(Frame=25)
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=83, KBG=65, FKB=0, BKB=90, Size=7.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=14.0, Z2=2.0, Hitlag=2.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=83, KBG=55, FKB=0, BKB=90, Size=7.0, X=0.0, Y=7.0, Z=2.0, X2=0.0, Y2=14.0, Z2=2.0, Hitlag=2.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=30)
        if(is_excute){
            MotionModule::set_rate(1.5)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_DOLLY,
    animation = "special_air_hi_command",
    animcmd = "game_specialairhicommand")]
pub fn dolly_uspecialaircommand(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR)
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH)
        }
        frame(Frame=9)
        if(is_excute){
            HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
            HIT_NODE(hash40("kneel"), HIT_STATUS_XLU)
            HIT_NODE(hash40("legr"), HIT_STATUS_XLU)
            HIT_NODE(hash40("legl"), HIT_STATUS_XLU)
            HIT_NODE(hash40("hip"), HIT_STATUS_XLU)
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.5, Angle=98, KBG=110, FKB=110, BKB=30, Size=4.5, X=0.0, Y=10.0, Z=6.5, X2=0.0, Y2=10.0, Z2=2.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.5, Angle=75, KBG=110, FKB=110, BKB=30, Size=4.5, X=0.0, Y=10.0, Z=-2.0, X2=0.0, Y2=10.0, Z2=-3.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            AttackModule::set_no_finish_camera(0, true, false)
            AttackModule::set_no_finish_camera(1, true, false)
            WorkModule::on_flag(Flag=FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        }
        frame(Frame=11)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=7.5, Z=0.0, X2=0.0, Y2=7.5, Z2=6.0, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=50, FKB=5, BKB=20, Size=3.5, X=0.0, Y=20.0, Z=1.0, X2=0.0, Y2=22.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=40, FKB=40, BKB=20, Size=4.5, X=0.0, Y=13.0, Z=3.0, X2=0.0, Y2=13.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=45, FKB=40, BKB=20, Size=4.5, X=0.0, Y=13.0, Z=3.0, X2=0.0, Y2=13.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=7.5, Z=-4.0, X2=0.0, Y2=7.5, Z2=-7.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
            WorkModule::off_flag(Flag=FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        }
        frame(Frame=13)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            MotionModule::set_rate(1.5)
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=6.5, Z=7.5, X2=0.0, Y2=6.5, Z2=0.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=50, FKB=5, BKB=20, Size=3.5, X=0.0, Y=19.0, Z=1.0, X2=0.0, Y2=21.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=20, Size=4.5, X=0.0, Y=12.0, Z=3.0, X2=0.0, Y2=12.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=45, FKB=40, BKB=20, Size=4.5, X=0.0, Y=12.0, Z=3.0, X2=0.0, Y2=12.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=6.5, Z=-4.0, X2=0.0, Y2=6.5, Z2=-4.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=5.5, Z=0.0, X2=0.0, Y2=5.5, Z2=6.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=50, FKB=5, BKB=20, Size=3.5, X=0.0, Y=18.0, Z=1.0, X2=0.0, Y2=20.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=20, Size=4.5, X=0.0, Y=11.0, Z=3.0, X2=0.0, Y2=11.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=45, FKB=40, BKB=20, Size=4.5, X=0.0, Y=11.0, Z=3.0, X2=0.0, Y2=11.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=-4.0, X2=0.0, Y2=5.5, Z2=-7.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=5.5, Z=7.5, X2=0.0, Y2=5.5, Z2=0.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=50, FKB=5, BKB=20, Size=3.5, X=0.0, Y=18.0, Z=1.0, X2=0.0, Y2=20.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=20, Size=4.5, X=0.0, Y=11.0, Z=3.0, X2=0.0, Y2=11.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=45, FKB=40, BKB=20, Size=4.5, X=0.0, Y=11.0, Z=3.0, X2=0.0, Y2=11.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=-4.0, X2=0.0, Y2=5.5, Z2=-4.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=21)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=5.5, Z=0.0, X2=0.0, Y2=5.5, Z2=6.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=50, FKB=5, BKB=20, Size=3.5, X=0.0, Y=18.0, Z=1.0, X2=0.0, Y2=20.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=20, Size=4.5, X=0.0, Y=11.0, Z=3.0, X2=0.0, Y2=11.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=45, FKB=40, BKB=20, Size=4.5, X=0.0, Y=11.0, Z=3.0, X2=0.0, Y2=11.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=5.5, Z=-4.0, X2=0.0, Y2=5.5, Z2=-7.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=24)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=4.5, Z=7.5, X2=0.0, Y2=4.5, Z2=0.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=50, FKB=5, BKB=20, Size=3.5, X=0.0, Y=17.0, Z=1.0, X2=0.0, Y2=19.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=35, FKB=30, BKB=20, Size=4.5, X=0.0, Y=10.0, Z=3.0, X2=0.0, Y2=10.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=45, FKB=40, BKB=20, Size=4.5, X=0.0, Y=10.0, Z=3.0, X2=0.0, Y2=10.0, Z2=1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.3, Angle=367, KBG=30, FKB=50, BKB=30, Size=3.0, X=0.0, Y=4.5, Z=-4.0, X2=0.0, Y2=4.5, Z2=-4.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legr"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("legl"), HIT_STATUS_NORMAL)
            HIT_NODE(hash40("hip"), HIT_STATUS_NORMAL)
            AttackModule::clear_all()
        }
        frame(Frame=29)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=30, FKB=50, BKB=30, Size=3.5, X=0.0, Y=3.5, Z=5.0, X2=0.0, Y2=3.5, Z2=-7.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.3, Angle=70, KBG=50, FKB=5, BKB=20, Size=3.5, X=0.0, Y=16.0, Z=1.0, X2=0.0, Y2=18.5, Z2=1.0, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=40, FKB=45, BKB=20, Size=4.5, X=0.0, Y=10.0, Z=3.0, X2=0.0, Y2=10.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.3, Angle=90, KBG=50, FKB=45, BKB=20, Size=4.5, X=0.0, Y=10.0, Z=3.0, X2=0.0, Y2=10.0, Z2=-1.5, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=34)
        if(is_excute){
            AttackModule::clear_all()
        }
        if(WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=83, KBG=73, FKB=0, BKB=90, Size=7.0, X=0.0, Y=8.0, Z=2.0, X2=0.0, Y2=15.0, Z2=2.0, Hitlag=2.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
            }
            else{
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=83, KBG=62, FKB=0, BKB=90, Size=7.0, X=0.0, Y=8.0, Z=2.0, X2=0.0, Y2=15.0, Z2=2.0, Hitlag=2.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_DOLLY_PUNCH, Type=ATTACK_REGION_BODY)
                }
            }
        }
        frame(Frame=35)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=40)
        if(is_excute){
            MotionModule::set_rate(1.5)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        dolly_dspecialair,
        dolly_uspecialair,
        dolly_uspecialaircommand
    );
}