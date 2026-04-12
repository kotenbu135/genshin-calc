use genshin_calc_data::find_character;

#[test]
fn qiqi_and_jahoda_scalings_match_honeyhunter_mirror() {
    let qiqi = find_character("qiqi").expect("Qiqi character data should exist");
    let qiqi_normal_1 = qiqi
        .talents
        .normal_attack
        .hits
        .iter()
        .find(|scaling| scaling.name == "1段ダメージ")
        .expect("Qiqi normal attack 1 scaling should exist");
    assert_eq!(
        qiqi_normal_1.values[0], 0.3775,
        "Qiqi normal attack 1 Lv1 should match honeyhunter mirror"
    );

    let jahoda = find_character("jahoda").expect("Jahoda character data should exist");
    let aimed = jahoda
        .talents
        .normal_attack
        .charged
        .iter()
        .find(|scaling| scaling.name == "狙い撃ち")
        .expect("Jahoda aimed shot scaling should exist");
    let aimed_full = jahoda
        .talents
        .normal_attack
        .charged
        .iter()
        .find(|scaling| scaling.name == "フルチャージ狙い撃ち")
        .expect("Jahoda fully charged aimed shot scaling should exist");

    assert_eq!(
        &aimed.values[10..15],
        &[0.9282, 0.9894, 1.0506, 1.1118, 1.1730],
        "Jahoda aimed shot Lv11-15 should match honeyhunter mirror"
    );
    assert_eq!(
        &aimed_full.values[10..15],
        &[2.356, 2.48, 2.635, 2.79, 2.945],
        "Jahoda fully charged aimed shot Lv11-15 should match honeyhunter mirror"
    );
}
