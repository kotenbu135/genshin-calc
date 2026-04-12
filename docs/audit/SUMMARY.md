# Character Audit Summary

全112キャラ（7元素）をhoneyhunter-mirror/mdと比較した結果。
追加ダメージ・元素チャージ系は対象外。

## Critical Bugs（値が間違い・計算結果に直接影響）

| キャラ | 元素 | 問題 | 詳細 |
|--------|------|------|------|
| Chiori | Geo | Skill/Burst DEFスケーリング全欠落 | ダメージ~55%過小評価。ATK部分のみ実装 |
| Diluc | Pyro | NA Lv11-15 全4段 + Charged Final スケーリング曲線エラー | 5星スケーリングカーブが違う。Lv1-10は正常 |
| Itto | Geo | NA Lv11-15 全4段 + Kesagiri スケーリング曲線エラー | 線形外挿を使用、実際のカーブと異なる。最大18%誤差 |
| Yoimiya | Pyro | NA N1B値間違い + ヒット配列構造ズレ | 68.38%→35.64%。配列全体がシフト |
| Thoma | Pyro | NA N4ヒット欠落 | 67.36% Lv1のヒットが配列にない |
| Ineffa | Electro | 突破ステータス間違い | CritDmg(0.384)→CritRate(0.192) |
| Sethos | Electro | Burst DMG Increase ScalingStat間違い | ScalingStat::Atk→ScalingStat::Em。HHはEM基準と明記 |
| Iansan | Electro | Skill Lv9 スケーリング値エラー | 4.6880→4.8688 |
| Varesa | Electro | Burst/Charged 3箇所値エラー | Flying Kick Lv8: 5.2190→5.5219, Lv14: 7.6550→7.7652, Passion Charged Lv10: 1.6780→1.6675 |
| Alhaitham | Dendro | C2 最大スタック数間違い | Stacks(3)→Stacks(4) |
| Nahida | Dendro | C4 EMスケーリングモデル間違い | 100/200/300/400→100/120/140/160 |

## Constellation Pattern Bugs（C3/C5の対象スキル入れ替わり）

| キャラ | 元素 | 現在 | 正しい値 |
|--------|------|------|----------|
| Gaming | Pyro | C3BurstC5Skill | C3SkillC5Burst |
| Klee | Pyro | C3BurstC5Skill | C3SkillC5Burst |
| Yoimiya | Pyro | C3BurstC5Skill | C3SkillC5Burst |
| Lyney | Pyro | C3SkillC5Burst | C3NormalC5Burst |
| Ayato | Hydro | C3BurstC5Skill | C3SkillC5Burst |
| Neuvillette | Hydro | C3BurstC5Skill | C3NormalC5Burst |
| Freminet | Cryo | C3NormalC5Burst | C3NormalC5Skill |
| Faruzan | Anemo | C3BurstC5Skill | C3SkillC5Burst |
| Heizou | Anemo | C3BurstC5Skill | C3SkillC5Burst |
| Ifa | Anemo | C3BurstC5Skill | C3SkillC5Burst |
| Kazuha | Anemo | C3BurstC5Skill | C3SkillC5Burst |
| Lan Yan | Anemo | C3BurstC5Skill | C3SkillC5Burst |
| Mizuki | Anemo | C3BurstC5Skill | C3SkillC5Burst |
| Sucrose | Anemo | C3BurstC5Skill | C3SkillC5Burst |
| Xiao | Anemo | C3BurstC5Skill | C3SkillC5Burst |

## Missing Scalings（スケーリング未実装）

| キャラ | 元素 | 問題 |
|--------|------|------|
| Alhaitham | Dendro | Skill/Burst EM部分未実装（Rush EM, 1-Mirror Projection ATK+EM, Burst Single-Instance EM） |
| Nahida | Dendro | Skill Tri-Karma ATK/EM部分未実装 |
| Nilou | Hydro | Whirling Steps 1-Hit/2-Hit/Water Wheel 未実装 |

## Missing Talent Buffs（バフ未実装）

### Pyro
- Amber: A1/A4 missing
- Bennett: C2 ER+30% missing
- Chevreuse: A4 missing

### Hydro
- Mona: C2/C4 説明不一致（要確認）
- Neuvillette: A1/A4 名前入れ替わり

### Cryo
- Shenhe: A4 Cryo RES shred 固定25%で実装（実際は16-25%タレントレベル依存）
- Skirk: Charged Lv8/Lv14 値エラー

### Electro
- Ororon: C2 基本8% Electro DMG Bonus 未実装（スタック分32%のみ）

### Anemo
- Ifa: A4 EM+80 missing
- Jean: C1 Skill DMG+40% missing
- Lynette: A1 Burst DMG+15% missing
- Sucrose: C6 Elemental DMG+20% missing
- Venti: C6 swirled element RES-20% missing

### Dendro
- Baizhu: A1 Dendro DMG+25% missing
- Kinich: C1 Scalespiker Cannon CRIT DMG+100% missing
- Lauma: A1 Bloom/Hyperbloom/Burgeon CRIT missing
- Nefer: A4 Skill/Burst DMG bonus per EM missing

## 各元素詳細レポート

- [Pyro](pyro.md)
- [Hydro](hydro.md)
- [Cryo](cryo.md)
- [Electro](electro.md)
- [Anemo](anemo.md)
- [Geo](geo.md)
- [Dendro](dendro.md)
