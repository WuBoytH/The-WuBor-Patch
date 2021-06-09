use arcropolis_api::{arc_callback, hash40};
use sli::SliFile;

#[arc_callback]
fn api_callback(hash: u64, data: &mut [u8]) -> Option<usize> {
    if hash == hash40("sound/param/soundlabelinfo.sli").as_u64() {
        println!("Calling soundlabelinfo.sli!");
        arcropolis_api::load_original_file(hash, data);
        let mut writer = std::io::Cursor::new(&data);
        let mut soundlabel = SliFile::read(&mut writer).unwrap();
        let mut counter = 0;
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("se_ryu_6c_aura").as_u64(),
            nus3bank_id: 2068,
            tone_id: 83
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("se_ryu_6c_exec").as_u64(),
            nus3bank_id: 2068,
            tone_id: 84
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_appeal01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 36
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_appeal02").as_u64(),
            nus3bank_id: 4118,
            tone_id: 37
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_attack01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 38
        });
        counter += 1;
        print!("{}", counter);
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
        counter += 1;
        print!("{}", counter);
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
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_attack06").as_u64(),
            nus3bank_id: 4118,
            tone_id: 43
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_attack07").as_u64(),
            nus3bank_id: 4118,
            tone_id: 44
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_cliffcatch").as_u64(),
            nus3bank_id: 4118,
            tone_id: 45
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_damage_twinkle").as_u64(),
            nus3bank_id: 4118,
            tone_id: 46
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_damage01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 47
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_damage02").as_u64(),
            nus3bank_id: 4118,
            tone_id: 48
        });
        counter += 1;
        print!("{}", counter);
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
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_damagefly02").as_u64(),
            nus3bank_id: 4118,
            tone_id: 51
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_final").as_u64(),
            nus3bank_id: 4118,
            tone_id: 52
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_furafura").as_u64(),
            nus3bank_id: 4118,
            tone_id: 53
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_furasleep").as_u64(),
            nus3bank_id: 4118,
            tone_id: 54
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_heavyget").as_u64(),
            nus3bank_id: 4118,
            tone_id: 55
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_jump01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 56
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_missfoot01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 57
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_missfoot02").as_u64(),
            nus3bank_id: 4118,
            tone_id: 58
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_ottotto").as_u64(),
            nus3bank_id: 4118,
            tone_id: 59
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_passive").as_u64(),
            nus3bank_id: 4118,
            tone_id: 60
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_special_h01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 61
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_special_l01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 62
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_special_l02").as_u64(),
            nus3bank_id: 4118,
            tone_id: 63
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_special_n01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 64
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_swimup").as_u64(),
            nus3bank_id: 4118,
            tone_id: 65
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_win01").as_u64(),
            nus3bank_id: 4118,
            tone_id: 66
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_win02").as_u64(),
            nus3bank_id: 4118,
            tone_id: 67
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_win03").as_u64(),
            nus3bank_id: 4118,
            tone_id: 68
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_win_marth").as_u64(),
            nus3bank_id: 4118,
            tone_id: 69
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_win_ike").as_u64(),
            nus3bank_id: 4118,
            tone_id: 70
        });
        counter += 1;
        print!("{}", counter);
        soundlabel.entries_mut().push(sli::Entry {
            tone_name: hash40("vc_shadow_knockout").as_u64(),
            nus3bank_id: 4118,
            tone_id: 71
        });
        counter += 1;
        print!("{}", counter);
        println!("Done!");
        soundlabel.write(&mut writer).unwrap();
        Some(writer.position() as usize)
    }
    else {
        None
    }
}

const SOUNDLABELSIZE: usize = 0x61A80;

pub fn install() {
    api_callback::install("sound/param/soundlabelinfo.sli", SOUNDLABELSIZE);
}