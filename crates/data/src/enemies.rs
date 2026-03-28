use crate::types::{EnemyData, ResistanceTemplate};

// ---------------------------------------------------------------------------
// Resistance Templates
// ---------------------------------------------------------------------------

/// 標準: 全耐性10% (ヒルチャール、宝盗団など大半の敵)
pub const ALL_10: ResistanceTemplate = ResistanceTemplate {
    name: "標準 (全耐性10%)",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 物理50% 元素10% (遺跡守衛、遺跡ハンターなど)
pub const PHYS_50_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理50% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.50,
};

/// 物理70% 元素10% (遺跡重機など)
pub const PHYS_70_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理70% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.70,
};

/// 物理30% 元素10% (ファデュイ先遣隊など)
pub const PHYS_30_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理30% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.30,
};

/// 物理-20% 元素10% (ファデュイ・デットエージェントなど)
pub const PHYS_MINUS20_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理-20% 元素10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: -0.20,
};

/// 炎70% 他10% (爆炎樹など)
pub const PYRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "炎70% 他10%",
    pyro: 0.70,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 氷70% 他10% (急凍樹など)
pub const CRYO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "氷70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.70,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷70% 他10% (雷音権現など)
pub const ELECTRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.70,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 水70% 他10% (純水精霊など)
pub const HYDRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "水70% 他10%",
    pyro: 0.10,
    hydro: 0.70,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 草70% 他10% (翠翎恐蕈など)
pub const DENDRO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "草70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.70,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 岩70% 他10% (無相の岩など)
pub const GEO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "岩70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.70,
    physical: 0.10,
};

/// 風70% 他10% (無相の風など)
pub const ANEMO_70_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "風70% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.70,
    geo: 0.10,
    physical: 0.10,
};

// ---------------------------------------------------------------------------
// 追加テンプレート: 週ボス・フィールドボス用
// ---------------------------------------------------------------------------

/// 水50% 他0% (タルタリヤ・第1段階)
pub const HYDRO_50_REST_0: ResistanceTemplate = ResistanceTemplate {
    name: "水50% 他0%",
    pyro: 0.0,
    hydro: 0.50,
    electro: 0.0,
    cryo: 0.0,
    dendro: 0.0,
    anemo: 0.0,
    geo: 0.0,
    physical: 0.0,
};

/// 雷50% 他0% (タルタリヤ・第2段階)
pub const ELECTRO_50_REST_0: ResistanceTemplate = ResistanceTemplate {
    name: "雷50% 他0%",
    pyro: 0.0,
    hydro: 0.0,
    electro: 0.50,
    cryo: 0.0,
    dendro: 0.0,
    anemo: 0.0,
    geo: 0.0,
    physical: 0.0,
};

/// 水70% 雷70% 他0% (タルタリヤ・第3段階)
pub const HYDRO_70_ELECTRO_70_REST_0: ResistanceTemplate = ResistanceTemplate {
    name: "水70% 雷70% 他0%",
    pyro: 0.0,
    hydro: 0.70,
    electro: 0.70,
    cryo: 0.0,
    dendro: 0.0,
    anemo: 0.0,
    geo: 0.0,
    physical: 0.0,
};

/// 岩70% 物理40% 他10% (若陀龍王)
pub const GEO_70_PHYS_40_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "岩70% 物理40% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.70,
    physical: 0.40,
};

/// 氷50% 他10% (シニョーラ・第1段階)
pub const CRYO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "氷50% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.50,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 炎70% 氷50% 他10% (シニョーラ・第2段階)
pub const PYRO_70_CRYO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "炎70% 氷50% 他10%",
    pyro: 0.70,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.50,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷50% 他10% (散兵・第1段階)
pub const ELECTRO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷50% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.50,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷90% 他30% (散兵・第2段階)
pub const ELECTRO_90_REST_30: ResistanceTemplate = ResistanceTemplate {
    name: "雷90% 他30%",
    pyro: 0.30,
    hydro: 0.30,
    electro: 0.90,
    cryo: 0.30,
    dendro: 0.30,
    anemo: 0.30,
    geo: 0.30,
    physical: 0.30,
};

/// 草50% 他10% (アペプ・第2段階)
pub const DENDRO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "草50% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.50,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷70% 物理30% 他10% (雷レジスヴァイン)
pub const ELECTRO_70_PHYS_30_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷70% 物理30% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.70,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.30,
};

/// 氷免疫 物理0% 他10% (無相の氷) — Immune: cryo = 10.0
pub const CRYO_IMMUNE_PHYS_0_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "氷免疫 物理0% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 10.0, // Immune
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.0,
};

/// 雷免疫 物理0% 他10% (無相の雷) — Immune: electro = 10.0
pub const ELECTRO_IMMUNE_PHYS_0_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷免疫 物理0% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 10.0, // Immune
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.0,
};

/// 草免疫 物理0% 他10% (無相の草) — Immune: dendro = 10.0
pub const DENDRO_IMMUNE_PHYS_0_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "草免疫 物理0% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 10.0, // Immune
    anemo: 0.10,
    geo: 0.10,
    physical: 0.0,
};

/// 全耐性の一覧
pub const ALL_TEMPLATES: &[&ResistanceTemplate] = &[
    // 既存12
    &ALL_10,
    &PHYS_50_ELEM_10,
    &PHYS_70_ELEM_10,
    &PHYS_30_ELEM_10,
    &PHYS_MINUS20_ELEM_10,
    &PYRO_70_REST_10,
    &CRYO_70_REST_10,
    &ELECTRO_70_REST_10,
    &HYDRO_70_REST_10,
    &DENDRO_70_REST_10,
    &GEO_70_REST_10,
    &ANEMO_70_REST_10,
    // 新規13
    &HYDRO_50_REST_0,
    &ELECTRO_50_REST_0,
    &HYDRO_70_ELECTRO_70_REST_0,
    &GEO_70_PHYS_40_REST_10,
    &CRYO_50_REST_10,
    &PYRO_70_CRYO_50_REST_10,
    &ELECTRO_50_REST_10,
    &ELECTRO_90_REST_30,
    &DENDRO_50_REST_10,
    &ELECTRO_70_PHYS_30_REST_10,
    &CRYO_IMMUNE_PHYS_0_REST_10,
    &ELECTRO_IMMUNE_PHYS_0_REST_10,
    &DENDRO_IMMUNE_PHYS_0_REST_10,
];

// ---------------------------------------------------------------------------
// Enemy Data
// ---------------------------------------------------------------------------

pub const HILICHURL: EnemyData = EnemyData {
    id: "hilichurl",
    name: "ヒルチャール",
    resistance: &ALL_10,
};

pub const MITACHURL: EnemyData = EnemyData {
    id: "mitachurl",
    name: "ヒルチャール暴徒",
    resistance: &ALL_10,
};

pub const LAWACHURL: EnemyData = EnemyData {
    id: "lawachurl",
    name: "ヒルチャール・岩兜の王",
    resistance: &ALL_10,
};

pub const TREASURE_HOARDER: EnemyData = EnemyData {
    id: "treasure_hoarder",
    name: "宝盗団",
    resistance: &ALL_10,
};

pub const FATUI_SKIRMISHER: EnemyData = EnemyData {
    id: "fatui_skirmisher",
    name: "ファデュイ先遣隊",
    resistance: &PHYS_30_ELEM_10,
};

pub const FATUI_AGENT: EnemyData = EnemyData {
    id: "fatui_agent",
    name: "ファデュイ・デットエージェント",
    resistance: &PHYS_MINUS20_ELEM_10,
};

pub const RUIN_GUARD: EnemyData = EnemyData {
    id: "ruin_guard",
    name: "遺跡守衛",
    resistance: &PHYS_50_ELEM_10,
};

pub const RUIN_HUNTER: EnemyData = EnemyData {
    id: "ruin_hunter",
    name: "遺跡ハンター",
    resistance: &PHYS_50_ELEM_10,
};

pub const RUIN_GRADER: EnemyData = EnemyData {
    id: "ruin_grader",
    name: "遺跡重機",
    resistance: &PHYS_70_ELEM_10,
};

pub const PYRO_REGISVINE: EnemyData = EnemyData {
    id: "pyro_regisvine",
    name: "爆炎樹",
    resistance: &PYRO_70_REST_10,
};

pub const CRYO_REGISVINE: EnemyData = EnemyData {
    id: "cryo_regisvine",
    name: "急凍樹",
    resistance: &CRYO_70_REST_10,
};

pub const THUNDER_MANIFESTATION: EnemyData = EnemyData {
    id: "thunder_manifestation",
    name: "雷音権現",
    resistance: &ELECTRO_70_REST_10,
};

pub const HYDRO_HYPOSTASIS: EnemyData = EnemyData {
    id: "hydro_hypostasis",
    name: "無相の水",
    resistance: &HYDRO_70_REST_10,
};

pub const GEO_HYPOSTASIS: EnemyData = EnemyData {
    id: "geo_hypostasis",
    name: "無相の岩",
    resistance: &GEO_70_REST_10,
};

pub const ANEMO_HYPOSTASIS: EnemyData = EnemyData {
    id: "anemo_hypostasis",
    name: "無相の風",
    resistance: &ANEMO_70_REST_10,
};

// ---------------------------------------------------------------------------
// 週ボス
// ---------------------------------------------------------------------------

pub const DVALIN: EnemyData = EnemyData {
    id: "dvalin",
    name: "風魔龍トワリン",
    resistance: &ALL_10,
};

pub const CHILDE_PHASE1: EnemyData = EnemyData {
    id: "childe_phase1",
    name: "タルタリヤ・第1段階",
    resistance: &HYDRO_50_REST_0,
};

pub const CHILDE_PHASE2: EnemyData = EnemyData {
    id: "childe_phase2",
    name: "タルタリヤ・第2段階",
    resistance: &ELECTRO_50_REST_0,
};

pub const CHILDE_PHASE3: EnemyData = EnemyData {
    id: "childe_phase3",
    name: "タルタリヤ・第3段階",
    resistance: &HYDRO_70_ELECTRO_70_REST_0,
};

pub const AZHDAHA: EnemyData = EnemyData {
    id: "azhdaha",
    name: "若陀龍王",
    resistance: &GEO_70_PHYS_40_REST_10,
};

pub const SIGNORA_PHASE1: EnemyData = EnemyData {
    id: "signora_phase1",
    name: "シニョーラ・第1段階",
    resistance: &CRYO_50_REST_10,
};

pub const SIGNORA_PHASE2: EnemyData = EnemyData {
    id: "signora_phase2",
    name: "シニョーラ・第2段階",
    resistance: &PYRO_70_CRYO_50_REST_10,
};

pub const RAIDEN_SHOGUN_WEEKLY: EnemyData = EnemyData {
    id: "raiden_shogun_weekly",
    name: "禍津御建鳴神命",
    resistance: &ALL_10,
};

pub const SCARAMOUCHE_PHASE1: EnemyData = EnemyData {
    id: "scaramouche_phase1",
    name: "正機の神・第1段階",
    resistance: &ELECTRO_50_REST_10,
};

pub const SCARAMOUCHE_PHASE2: EnemyData = EnemyData {
    id: "scaramouche_phase2",
    name: "正機の神・第2段階",
    resistance: &ELECTRO_90_REST_30,
};

pub const APEP_PHASE1: EnemyData = EnemyData {
    id: "apep_phase1",
    name: "アペプ・第1段階",
    resistance: &DENDRO_70_REST_10,
};

pub const APEP_PHASE2: EnemyData = EnemyData {
    id: "apep_phase2",
    name: "アペプ・第2段階",
    resistance: &DENDRO_50_REST_10,
};

pub const APEP_PHASE3: EnemyData = EnemyData {
    id: "apep_phase3",
    name: "アペプ・第3段階",
    resistance: &DENDRO_70_REST_10,
};

pub const NARWHAL_PHASE1: EnemyData = EnemyData {
    id: "narwhal_phase1",
    name: "呑星の鯨・第1段階",
    resistance: &HYDRO_70_REST_10,
};

pub const NARWHAL_PHASE2: EnemyData = EnemyData {
    id: "narwhal_phase2",
    name: "呑星の鯨・第2段階",
    resistance: &ELECTRO_70_REST_10,
};

// ---------------------------------------------------------------------------
// フィールドボス
// ---------------------------------------------------------------------------

pub const ELECTRO_REGISVINE: EnemyData = EnemyData {
    id: "electro_regisvine",
    name: "雷レジスヴァイン",
    resistance: &ELECTRO_70_PHYS_30_REST_10,
};

pub const MAGUU_KENKI: EnemyData = EnemyData {
    id: "maguu_kenki",
    name: "魔偶剣鬼",
    resistance: &ALL_10,
};

pub const CRYO_HYPOSTASIS: EnemyData = EnemyData {
    id: "cryo_hypostasis",
    name: "無相の氷",
    resistance: &CRYO_IMMUNE_PHYS_0_REST_10,
};

pub const ELECTRO_HYPOSTASIS: EnemyData = EnemyData {
    id: "electro_hypostasis",
    name: "無相の雷",
    resistance: &ELECTRO_IMMUNE_PHYS_0_REST_10,
};

pub const DENDRO_HYPOSTASIS: EnemyData = EnemyData {
    id: "dendro_hypostasis",
    name: "無相の草",
    resistance: &DENDRO_IMMUNE_PHYS_0_REST_10,
};

// ---------------------------------------------------------------------------
// 精鋭
// ---------------------------------------------------------------------------

pub const ABYSS_HERALD: EnemyData = EnemyData {
    id: "abyss_herald",
    name: "アビスの使徒",
    resistance: &ALL_10,
};

pub const ABYSS_LECTOR: EnemyData = EnemyData {
    id: "abyss_lector",
    name: "アビスの詠唱者",
    resistance: &ALL_10,
};

pub const RUIN_DRAKE_SKYWATCH: EnemyData = EnemyData {
    id: "ruin_drake_skywatch",
    name: "遺跡ドレイク・飛空",
    resistance: &PHYS_50_ELEM_10,
};

pub const RUIN_DRAKE_EARTHGUARD: EnemyData = EnemyData {
    id: "ruin_drake_earthguard",
    name: "遺跡ドレイク・陸行",
    resistance: &PHYS_50_ELEM_10,
};

pub const EREMITE_ELITE: EnemyData = EnemyData {
    id: "eremite_elite",
    name: "エルマイト旅団・精鋭",
    resistance: &PHYS_MINUS20_ELEM_10,
};

/// 全敵データの一覧
pub const ALL_ENEMIES: &[&EnemyData] = &[
    // 既存15
    &HILICHURL,
    &MITACHURL,
    &LAWACHURL,
    &TREASURE_HOARDER,
    &FATUI_SKIRMISHER,
    &FATUI_AGENT,
    &RUIN_GUARD,
    &RUIN_HUNTER,
    &RUIN_GRADER,
    &PYRO_REGISVINE,
    &CRYO_REGISVINE,
    &THUNDER_MANIFESTATION,
    &HYDRO_HYPOSTASIS,
    &GEO_HYPOSTASIS,
    &ANEMO_HYPOSTASIS,
    // 週ボス
    &DVALIN,
    &CHILDE_PHASE1,
    &CHILDE_PHASE2,
    &CHILDE_PHASE3,
    &AZHDAHA,
    &SIGNORA_PHASE1,
    &SIGNORA_PHASE2,
    &RAIDEN_SHOGUN_WEEKLY,
    &SCARAMOUCHE_PHASE1,
    &SCARAMOUCHE_PHASE2,
    &APEP_PHASE1,
    &APEP_PHASE2,
    &APEP_PHASE3,
    &NARWHAL_PHASE1,
    &NARWHAL_PHASE2,
    // フィールドボス
    &ELECTRO_REGISVINE,
    &MAGUU_KENKI,
    &CRYO_HYPOSTASIS,
    &ELECTRO_HYPOSTASIS,
    &DENDRO_HYPOSTASIS,
    // 精鋭
    &ABYSS_HERALD,
    &ABYSS_LECTOR,
    &RUIN_DRAKE_SKYWATCH,
    &RUIN_DRAKE_EARTHGUARD,
    &EREMITE_ELITE,
];

#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::Element;

    #[test]
    fn test_all_10_template() {
        let eps = 1e-10;
        assert!((ALL_10.pyro - 0.10).abs() < eps);
        assert!((ALL_10.physical - 0.10).abs() < eps);
        assert!((ALL_10.get(Some(Element::Pyro)) - 0.10).abs() < eps);
        assert!((ALL_10.get(None) - 0.10).abs() < eps);
    }

    #[test]
    fn test_phys_50_template() {
        let eps = 1e-10;
        assert!((PHYS_50_ELEM_10.physical - 0.50).abs() < eps);
        assert!((PHYS_50_ELEM_10.pyro - 0.10).abs() < eps);
        assert!((PHYS_50_ELEM_10.get(None) - 0.50).abs() < eps);
        assert!((PHYS_50_ELEM_10.get(Some(Element::Hydro)) - 0.10).abs() < eps);
    }

    #[test]
    fn test_negative_resistance_template() {
        let eps = 1e-10;
        assert!((PHYS_MINUS20_ELEM_10.physical - (-0.20)).abs() < eps);
        assert!((PHYS_MINUS20_ELEM_10.get(None) - (-0.20)).abs() < eps);
    }

    #[test]
    fn test_enemy_to_core() {
        let enemy = RUIN_GUARD.to_enemy(Some(Element::Pyro), 90, 0.0);
        let eps = 1e-10;
        assert_eq!(enemy.level, 90);
        assert!((enemy.resistance - 0.10).abs() < eps);
        assert!((enemy.def_reduction - 0.0).abs() < eps);
    }

    #[test]
    fn test_enemy_physical_resistance() {
        let enemy = RUIN_GUARD.to_enemy(None, 85, 0.0);
        let eps = 1e-10;
        assert!((enemy.resistance - 0.50).abs() < eps);
    }

    #[test]
    fn test_all_enemies_unique_ids() {
        for (i, a) in ALL_ENEMIES.iter().enumerate() {
            for (j, b) in ALL_ENEMIES.iter().enumerate() {
                if i != j {
                    assert_ne!(a.id, b.id, "Duplicate enemy id: {}", a.id);
                }
            }
        }
    }

    #[test]
    fn test_all_templates_count() {
        assert_eq!(ALL_TEMPLATES.len(), 25);
    }

    #[test]
    fn test_all_enemies_count() {
        assert_eq!(ALL_ENEMIES.len(), 40);
    }

    #[test]
    fn test_hydro_50_rest_0_template() {
        let eps = 1e-10;
        assert!((HYDRO_50_REST_0.hydro - 0.50).abs() < eps);
        assert!((HYDRO_50_REST_0.physical - 0.0).abs() < eps);
        assert!((HYDRO_50_REST_0.pyro - 0.0).abs() < eps);
    }

    #[test]
    fn test_hydro_70_electro_70_rest_0_template() {
        let eps = 1e-10;
        assert!((HYDRO_70_ELECTRO_70_REST_0.hydro - 0.70).abs() < eps);
        assert!((HYDRO_70_ELECTRO_70_REST_0.electro - 0.70).abs() < eps);
        assert!((HYDRO_70_ELECTRO_70_REST_0.physical - 0.0).abs() < eps);
        assert!((HYDRO_70_ELECTRO_70_REST_0.pyro - 0.0).abs() < eps);
    }

    #[test]
    fn test_geo_70_phys_40_rest_10_template() {
        let eps = 1e-10;
        assert!((GEO_70_PHYS_40_REST_10.geo - 0.70).abs() < eps);
        assert!((GEO_70_PHYS_40_REST_10.physical - 0.40).abs() < eps);
        assert!((GEO_70_PHYS_40_REST_10.pyro - 0.10).abs() < eps);
    }

    #[test]
    fn test_pyro_70_cryo_50_rest_10_template() {
        let eps = 1e-10;
        assert!((PYRO_70_CRYO_50_REST_10.pyro - 0.70).abs() < eps);
        assert!((PYRO_70_CRYO_50_REST_10.cryo - 0.50).abs() < eps);
        assert!((PYRO_70_CRYO_50_REST_10.hydro - 0.10).abs() < eps);
    }

    #[test]
    fn test_electro_90_rest_30_template() {
        let eps = 1e-10;
        assert!((ELECTRO_90_REST_30.electro - 0.90).abs() < eps);
        assert!((ELECTRO_90_REST_30.physical - 0.30).abs() < eps);
        assert!((ELECTRO_90_REST_30.pyro - 0.30).abs() < eps);
    }

    #[test]
    fn test_immune_templates() {
        let eps = 1e-10;
        // Immune = 10.0 (1000% resistance)
        assert!((CRYO_IMMUNE_PHYS_0_REST_10.cryo - 10.0).abs() < eps);
        assert!((CRYO_IMMUNE_PHYS_0_REST_10.physical - 0.0).abs() < eps);
        assert!((CRYO_IMMUNE_PHYS_0_REST_10.pyro - 0.10).abs() < eps);

        assert!((ELECTRO_IMMUNE_PHYS_0_REST_10.electro - 10.0).abs() < eps);
        assert!((ELECTRO_IMMUNE_PHYS_0_REST_10.physical - 0.0).abs() < eps);

        assert!((DENDRO_IMMUNE_PHYS_0_REST_10.dendro - 10.0).abs() < eps);
        assert!((DENDRO_IMMUNE_PHYS_0_REST_10.physical - 0.0).abs() < eps);
    }

    #[test]
    fn test_childe_phase1_hydro_resistance() {
        let eps = 1e-10;
        let enemy = CHILDE_PHASE1.to_enemy(Some(Element::Hydro), 90, 0.0);
        assert!((enemy.resistance - 0.50).abs() < eps);
    }

    #[test]
    fn test_childe_phase1_non_hydro_resistance() {
        let eps = 1e-10;
        let enemy = CHILDE_PHASE1.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.0).abs() < eps);
    }

    #[test]
    fn test_childe_phase3_dual_resistance() {
        let eps = 1e-10;
        let enemy = CHILDE_PHASE3.to_enemy(Some(Element::Hydro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = CHILDE_PHASE3.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = CHILDE_PHASE3.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.0).abs() < eps);
    }

    #[test]
    fn test_azhdaha_geo_and_physical() {
        let eps = 1e-10;
        let enemy = AZHDAHA.to_enemy(Some(Element::Geo), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = AZHDAHA.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - 0.40).abs() < eps);
    }

    #[test]
    fn test_signora_phase2_dual_element() {
        let eps = 1e-10;
        let enemy = SIGNORA_PHASE2.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = SIGNORA_PHASE2.to_enemy(Some(Element::Cryo), 90, 0.0);
        assert!((enemy.resistance - 0.50).abs() < eps);
    }

    #[test]
    fn test_scaramouche_phase2_high_electro() {
        let eps = 1e-10;
        let enemy = SCARAMOUCHE_PHASE2.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 0.90).abs() < eps);
        let enemy = SCARAMOUCHE_PHASE2.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.30).abs() < eps);
    }

    #[test]
    fn test_cryo_hypostasis_immune() {
        let eps = 1e-10;
        let enemy = CRYO_HYPOSTASIS.to_enemy(Some(Element::Cryo), 90, 0.0);
        assert!((enemy.resistance - 10.0).abs() < eps);
        // Non-immune element
        let enemy = CRYO_HYPOSTASIS.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.10).abs() < eps);
        // Physical
        let enemy = CRYO_HYPOSTASIS.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - 0.0).abs() < eps);
    }

    #[test]
    fn test_electro_hypostasis_immune() {
        let eps = 1e-10;
        let enemy = ELECTRO_HYPOSTASIS.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 10.0).abs() < eps);
    }

    #[test]
    fn test_dendro_hypostasis_immune() {
        let eps = 1e-10;
        let enemy = DENDRO_HYPOSTASIS.to_enemy(Some(Element::Dendro), 90, 0.0);
        assert!((enemy.resistance - 10.0).abs() < eps);
    }

    #[test]
    fn test_electro_regisvine_resistances() {
        let eps = 1e-10;
        let enemy = ELECTRO_REGISVINE.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = ELECTRO_REGISVINE.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - 0.30).abs() < eps);
    }

    #[test]
    fn test_eremite_negative_physical() {
        let eps = 1e-10;
        let enemy = EREMITE_ELITE.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - (-0.20)).abs() < eps);
        let enemy = EREMITE_ELITE.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.10).abs() < eps);
    }
}
