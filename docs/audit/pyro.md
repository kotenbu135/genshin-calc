# Pyro Audit — Issues Only

## Diluc
- [ ] **NA Lv11-15 全4段 スケーリング曲線エラー** — 5星カーブ間違い、Lv1-10正常
  - N1 Lv11: 189.89%→191.65%, N2: 185.46%→187.24%, N3: 209.17%→211.13%, N4: 283.62%→286.28%
  - Lv12-15も同パターン（全体的に低い）
  - File: `characters/pyro/diluc.rs`

## Yoimiya
- [ ] **NA N1B値間違い + ヒット配列構造崩壊**
  - N1B: code 68.38%→正しくは35.64%（N1Aと同値、HH "35.64%×2"）
  - 68.38%は実際のN2値。配列全体がシフト
  - 正: [N1A, N1B(=N1A), N2, N3, N4A, N4B, N5]（7エントリ）、現: 6エントリ
  - File: `characters/pyro/yoimiya.rs`
- [ ] **constellation_pattern**: `C3BurstC5Skill`→`C3SkillC5Burst`（C3=Niwabi Fire-Dance/Skill, C5=Ryuukin Saxifrage/Burst）
- [ ] **A1 未実装**: Pyro DMG+2%/stack max 10 during Niwabi Fire-Dance

## Thoma
- [ ] **NA N4 ヒット欠落** — 67.36% Lv1。配列にN1,N2,N3A,N3Bのみ
  - File: `characters/pyro/thoma.rs`

## Gaming
- [ ] **constellation_pattern**: `C3BurstC5Skill`→`C3SkillC5Burst`（C3=Bestial Ascent/Skill, C5=Suanni's Gilded Dance/Burst）

## Klee
- [ ] **constellation_pattern**: `C3BurstC5Skill`→`C3SkillC5Burst`（C3=Jumpy Dumpty/Skill, C5=Sparks 'n' Splash/Burst）

## Lyney
- [ ] **constellation_pattern**: `C3SkillC5Burst`→`C3NormalC5Burst`（C3=Card Force Translocation/Normal, C5=Magic Trick/Burst）

## Amber
- [ ] **A1未実装**: Burst CR+10%（2nd wave）
- [ ] **A4未実装**: ATK+15% 10s on weak point hit

## Bennett
- [ ] **C2未実装**: ER+30%（HP<70%）— utility系、意図的skipの可能性

## Chevreuse
- [ ] **A4未実装**: ATK+20% for Pyro/Electro chars on HP-based Overloaded
