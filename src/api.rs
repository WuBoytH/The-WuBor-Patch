use arcropolis_api::{arc_callback, hash40, load_original_file};
use sli::SliFile;
use prc::*;
use prc::hash40::{Hash40, to_hash40};

#[arc_callback]
fn api_callback(hash: u64, data: &mut [u8]) -> Option<usize> {
    if hash == hash40("ui/param/database/ui_chara_db.prc").as_u64() {
        if load_original_file(hash, data.as_mut()).is_some() {
            // skyline_web::DialogOk::ok("ui_chara_db.prc ain't shit");
            let mut reader = std::io::Cursor::new(data);
            let mut root = prc::read_stream(&mut reader).unwrap();
            let (db_root_hash, db_root) = &mut root.0[0];
            assert_eq!(*db_root_hash, to_hash40("db_root"));
            let db_root_list = db_root.try_into_mut::<ParamList>().unwrap();
            db_root_list.0.iter_mut().for_each(|param| {
                println!("Hi");
                let ui_chara_struct = param.try_into_ref::<ParamStruct>().unwrap();
                let (_, ui_chara_id) = &ui_chara_struct.0[0];
                let ui_chara_hash = ui_chara_id.try_into_ref::<Hash40>().unwrap();
                if *ui_chara_hash == to_hash40("ui_chara_donkey") {
                    let donkey = param.try_into_mut::<ParamStruct>().unwrap();
                    donkey.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("n02_index") {
                            *param.try_into_mut::<u8>().unwrap() = 2;
                        }
                        else if *hash == to_hash40("n04_index") {
                            *param.try_into_mut::<u8>().unwrap() = 4;
                        }
                        else if *hash == to_hash40("characall_label_c02") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling_c01_article");
                        }
                        else if *hash == to_hash40("characall_label_c04") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_samusd") {
                    let samusd = param.try_into_mut::<ParamStruct>().unwrap();
                    samusd.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_ice_climber") {
                    let iceclimber = param.try_into_mut::<ParamStruct>().unwrap();
                    iceclimber.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_lucina") {
                    let lucina = param.try_into_mut::<ParamStruct>().unwrap();
                    lucina.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("ui_series_id") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_persona");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_younglink") {
                    let ylink = param.try_into_mut::<ParamStruct>().unwrap();
                    ylink.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_pitb") {
                    let pitb = param.try_into_mut::<ParamStruct>().unwrap();
                    pitb.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_snake") {
                    let snake = param.try_into_mut::<ParamStruct>().unwrap();
                    snake.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("n00_index") {
                            *param.try_into_mut::<u8>().unwrap() = 2;
                        }
                        else if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling_c01");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_ike") {
                    let ike = param.try_into_mut::<ParamStruct>().unwrap();
                    ike.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("n01_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        else if *hash == to_hash40("n03_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        else if *hash == to_hash40("n05_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        else if *hash == to_hash40("n07_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave_article");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_ptrainer") {
                    let ptrainer = param.try_into_mut::<ParamStruct>().unwrap();
                    ptrainer.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_murabito") {
                    let murabito = param.try_into_mut::<ParamStruct>().unwrap();
                    murabito.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_wiifit") {
                    let wiifit = param.try_into_mut::<ParamStruct>().unwrap();
                    wiifit.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_reflet") {
                    let reflet = param.try_into_mut::<ParamStruct>().unwrap();
                    reflet.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_reflet");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_duckhunt") {
                    let duck = param.try_into_mut::<ParamStruct>().unwrap();
                    duck.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_bayonetta") {
                    let bayo = param.try_into_mut::<ParamStruct>().unwrap();
                    bayo.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("n01_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        else if *hash == to_hash40("n03_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        else if *hash == to_hash40("n05_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        else if *hash == to_hash40("n07_index") {
                            *param.try_into_mut::<u8>().unwrap() = 1;
                        }
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_dedede_article");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_inkling") {
                    let inkling = param.try_into_mut::<ParamStruct>().unwrap();
                    inkling.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_krool") {
                    let krool = param.try_into_mut::<ParamStruct>().unwrap();
                    krool.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_miifighter") {
                    let brawler = param.try_into_mut::<ParamStruct>().unwrap();
                    brawler.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter_c01");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter_c01");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter_c01");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_miiswordsman") {
                    let swordsman = param.try_into_mut::<ParamStruct>().unwrap();
                    swordsman.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_c01");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_c01");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_c01");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_miigunner") {
                    let gunner = param.try_into_mut::<ParamStruct>().unwrap();
                    gunner.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner_c01");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner_c01");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner_c01");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_packun") {
                    let packun = param.try_into_mut::<ParamStruct>().unwrap();
                    packun.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_brave") {
                    let brave = param.try_into_mut::<ParamStruct>().unwrap();
                    brave.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c02") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer_article");
                        }
                        else if *hash == to_hash40("characall_label_article_c00") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave");
                        }
                        else if *hash == to_hash40("characall_label_article_c01") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave");
                        }
                        else if *hash == to_hash40("characall_label_article_c02") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer_article");
                        }
                        else if *hash == to_hash40("characall_label_article_c03") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_dolly") {
                    let dolly = param.try_into_mut::<ParamStruct>().unwrap();
                    dolly.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("n04_index") {
                            *param.try_into_mut::<u8>().unwrap() = 4;
                        }
                        else if *hash == to_hash40("n05_index") {
                            *param.try_into_mut::<u8>().unwrap() = 5;
                        }
                        else if *hash == to_hash40("characall_label_c04") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_reflet_c01");
                        }
                        else if *hash == to_hash40("characall_label_c05") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer_c01");
                        }
                    });
                }
                else if *ui_chara_hash == to_hash40("ui_chara_pickel") {
                    let pickel = param.try_into_mut::<ParamStruct>().unwrap();
                    pickel.0.iter_mut().for_each(|(hash, param)| {
                        if *hash == to_hash40("characall_label_c06") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pickel_c06");
                        }
                        else if *hash == to_hash40("characall_label_c07") {
                            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pickel_c07");
                        }
                    });
                }
            });
            println!("Done parsing! Writer time...");
            let mut writer = std::io::Cursor::new(reader.into_inner());
            println!("Writer made! time to write to stream...");
            match prc::write_stream(&mut writer, &root) {
                Ok(_) => {
                    println!("Heeeeeey, that's pretty good!");
                    return Some(writer.position() as usize);
                }
                Err(_) => {
                    println!("fuq");
                    return None;
                }
            }
        }
        else {
            // skyline_web::DialogOk::ok("Damn, ui_chara_db.prc got hands");
            return None;
        }
    }
    if hash == hash40("sound/param/soundlabelinfo.sli").as_u64() {
        if load_original_file(hash, data.as_mut()).is_some() {
            let mut reader = std::io::Cursor::new(data);
            let mut soundlabel = SliFile::read(&mut reader).unwrap();
            let test = sli::Hash40{};
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("se_ryu_6c_aura").as_u64(),
                nus3bank_id: 2068,
                tone_id: 83
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("se_ryu_6c_exec").as_u64(),
                nus3bank_id: 2068,
                tone_id: 84
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_appeal01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 36
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_appeal02").as_u64(),
                nus3bank_id: 4118,
                tone_id: 37
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_attack01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 38
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_attack02").as_u64(),
                nus3bank_id: 4118,
                tone_id: 39
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_attack03").as_u64(),
                nus3bank_id: 4118,
                tone_id: 40
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_attack04").as_u64(),
                nus3bank_id: 4118,
                tone_id: 41
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_attack05").as_u64(),
                nus3bank_id: 4118,
                tone_id: 42
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_attack06").as_u64(),
                nus3bank_id: 4118,
                tone_id: 43
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_attack07").as_u64(),
                nus3bank_id: 4118,
                tone_id: 44
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_cliffcatch").as_u64(),
                nus3bank_id: 4118,
                tone_id: 45
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_damage_twinkle").as_u64(),
                nus3bank_id: 4118,
                tone_id: 46
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_damage01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 47
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_damage02").as_u64(),
                nus3bank_id: 4118,
                tone_id: 48
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_damage03").as_u64(),
                nus3bank_id: 4118,
                tone_id: 49
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_damagefly01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 50
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_damagefly02").as_u64(),
                nus3bank_id: 4118,
                tone_id: 51
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_final").as_u64(),
                nus3bank_id: 4118,
                tone_id: 52
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_furafura").as_u64(),
                nus3bank_id: 4118,
                tone_id: 53
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_furasleep").as_u64(),
                nus3bank_id: 4118,
                tone_id: 54
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_heavyget").as_u64(),
                nus3bank_id: 4118,
                tone_id: 55
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_jump01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 56
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_missfoot01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 57
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_missfoot02").as_u64(),
                nus3bank_id: 4118,
                tone_id: 58
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_ottotto").as_u64(),
                nus3bank_id: 4118,
                tone_id: 59
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_passive").as_u64(),
                nus3bank_id: 4118,
                tone_id: 60
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_special_h01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 61
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_special_l01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 62
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_special_l02").as_u64(),
                nus3bank_id: 4118,
                tone_id: 63
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_special_n01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 64
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_swimup").as_u64(),
                nus3bank_id: 4118,
                tone_id: 65
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_win01").as_u64(),
                nus3bank_id: 4118,
                tone_id: 66
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_win02").as_u64(),
                nus3bank_id: 4118,
                tone_id: 67
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_win03").as_u64(),
                nus3bank_id: 4118,
                tone_id: 68
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_win_marth").as_u64(),
                nus3bank_id: 4118,
                tone_id: 69
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_win_ike").as_u64(),
                nus3bank_id: 4118,
                tone_id: 70
            });
            soundlabel.entries_mut().push(sli::Entry {
                tone_name: hash40("vc_shadow_knockout").as_u64(),
                nus3bank_id: 4118,
                tone_id: 71
            });
    
            match soundlabel.write(&mut reader)
            {
                Ok(_f) => {
                    println!("Done with sli!");
                    return Some(reader.position() as usize);
                }
                Err(_e) => {
                    return None;
                }
            }
        }
        else {
            return None;
        }
    }
    else {
        None
    }
}

const SOUNDLABELSIZE: usize = 0x71A80;
const CHARASIZE: usize = 0x61A80;

pub fn install() {
    // api_callback::install("ui/param/database/ui_chara_db.prc", CHARASIZE);
    api_callback::install("sound/param/soundlabelinfo.sli", SOUNDLABELSIZE);
}