use genshin_calc_data::find_character;

#[test]
fn yae_miko_cyno_lisa_spot_scalings_match_honeyhunter_mirror() {
    let yae_miko = find_character("yae_miko").expect("Yae Miko character data should exist");
    let yae_skill_lv3 = yae_miko
        .talents
        .elemental_skill
        .scalings
        .iter()
        .find(|scaling| scaling.name == "殺生桜ダメージ・参階")
        .expect("Yae Miko Sesshou Sakura Level 3 scaling should exist");
    assert_eq!(
        yae_skill_lv3.values[14], 2.2515,
        "Yae Miko Sesshou Sakura Level 3 Lv15 should match honeyhunter mirror"
    );

    let cyno = find_character("cyno").expect("Cyno character data should exist");
    let cyno_normal_3 = cyno
        .talents
        .normal_attack
        .hits
        .iter()
        .find(|scaling| scaling.name == "3段ダメージ")
        .expect("Cyno Normal Attack 3 scaling should exist");
    assert_eq!(
        cyno_normal_3.values[9], 1.1586,
        "Cyno Normal Attack 3 Lv10 should match the combined honeyhunter mirror value"
    );

    let lisa = find_character("lisa").expect("Lisa character data should exist");
    let lisa_skill_hold_3stack = lisa
        .talents
        .elemental_skill
        .scalings
        .iter()
        .find(|scaling| scaling.name == "長押しダメージ (3重)")
        .expect("Lisa hold 3-stack skill scaling should exist");
    assert_eq!(
        lisa_skill_hold_3stack.values[7], 7.7952,
        "Lisa hold 3-stack Lv8 should match honeyhunter mirror"
    );
}
