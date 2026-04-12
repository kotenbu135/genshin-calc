# Geo Audit — Issues Only

## Chiori（CRITICAL）
- [ ] **Skill/Burst DEFスケーリング全欠落** — ATK部分のみ実装、DEF部分（~55%のダメージ）未実装
  - Tamoto: 82.08% ATK実装済み、**102.6% DEF** Lv1 未実装
  - Upward Sweep: 149.28% ATK実装済み、**186.6% DEF** Lv1 未実装
  - Burst: 256.32% ATK実装済み、**320.4% DEF** Lv1 未実装
  - 全15Lv分欠落

## Itto
- [ ] **NA Lv11-15 スケーリング曲線エラー** — 線形外挿、実際のカーブと異なる
  - N1 Lv11: 167.63%→169.29%, Lv15: 211.99%→230.23%（最大18%誤差）
  - N2-N4 + Charged Slash/Final 同パターン
  - Saichimonji/Plunge正常
- [ ] **C6**: `CritDmg`→Charged-specific CritDmg（HH: "Charged Attacks deal +70% CRIT DMG"）

## Albedo
- [ ] **A1未実装**: Skill DMG+25% vs <50% HP enemies
- [ ] **C2未実装**: Burst/Fatal Blossom DMG +30% DEF per stack (max 4)
- [ ] **"Witch's Eve Rite"未実装**: DMG +4%/10% per 1000 DEF

## Gorou
- [ ] **A1未実装**: Skill DMG +156% DEF, Burst DMG +15.6% DEF

## Zhongli
- [ ] **A4未実装**: HP-based bonus DMG（NA/CA/Plunge +1.39% HP, Skill +1.9% HP, Burst +33% HP）

## Xilonen
- [ ] **A1未実装**: NA/Plunge DMG+30%（conditional）

## Kachina
- [ ] **A4未実装**: Turbo Twirly DMG +20% DEF
- A1 naming: "Flowy Mountain"→"Mountain Echoes"

## Zibai
- [ ] **A1未実装**: Spirit Steed 2nd hit +60% DEF
- [ ] **A4未実装**: DEF+15% per Geo, EM+60 per Hydro

## Navia（Note）
- A1説明 "after burst"→"after Skill"。値正しい
