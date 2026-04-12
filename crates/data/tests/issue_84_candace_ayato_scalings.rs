use genshin_calc_data::find_character;

#[test]
fn candace_and_ayato_skill_scalings_match_honeyhunter_mirror() {
    let candace = find_character("candace").expect("Candace character data should exist");
    let candace_skill_press = candace
        .talents
        .elemental_skill
        .scalings
        .iter()
        .find(|scaling| scaling.name == "一段チャージダメージ")
        .expect("Candace skill press scaling should exist");

    let candace_expected = [
        0.1200, 0.1290, 0.1380, 0.1500, 0.1590, 0.1680, 0.1800, 0.1920, 0.2040, 0.2160, 0.2280,
        0.2400, 0.2550, 0.2700, 0.2850,
    ];
    assert_eq!(
        candace_skill_press.values, candace_expected,
        "Candace skill press Lv1-15 should match honeyhunter mirror"
    );

    let ayato = find_character("ayato").expect("Ayato character data should exist");
    let ayato_skill_3hit = ayato
        .talents
        .elemental_skill
        .scalings
        .iter()
        .find(|scaling| scaling.name == "瞬水剣3段ダメージ")
        .expect("Ayato skill 3-hit scaling should exist");
    assert_eq!(
        ayato_skill_3hit.values[7], 1.1099,
        "Ayato skill 3-hit Lv8 should match honeyhunter mirror"
    );
}
