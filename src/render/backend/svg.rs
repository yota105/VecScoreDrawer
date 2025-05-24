use crate::score::score_def_data::ScoreDef;
use std::collections::{HashMap, HashSet};
use regex::Regex;

/// SVGレンダリング関数
/// - score_def: YAMLからデシリアライズしたScoreDef
/// - pvsc_content: parsed_vsc.pvscの内容
/// - output_path: 出力SVGファイル名
pub fn render_svg(score_def: &ScoreDef, pvsc_content: &str, output_path: &str) -> anyhow::Result<()> {
    use svg::node::element::{Group, Line, Circle};
    use svg::Document;
    use num_integer::gcd;

    // 最初のパート・最初の小節のnotesを抽出
    let part = score_def.score.parts.get(0)
        .ok_or_else(|| anyhow::anyhow!("パートが見つかりません"))?;
    let measure_num = 1;
    let notes: Vec<_> = part.notes.iter().filter(|n| n.measure == measure_num).collect();

    // pvscからduration情報を取得
    let re_measure = Regex::new(r#"Measure \{\s+number: (\d+),"#).unwrap();
    let re_event = Regex::new(r#"Event \{\s+id: Some\(\s*(\d+)\s*\),[\s\S]+?duration: Ratio \{\s+numer: (\d+),\s+denom: (\d+),"#).unwrap();

    // (measure, id) -> (numer, denom)
    let mut duration_map = HashMap::new();
    let mut current_measure = 0;
    for cap in re_measure.captures_iter(pvsc_content) {
        current_measure = cap[1].parse::<usize>().unwrap_or(0);
        // イベントをこのmeasure内で探す
        let measure_start = cap.get(0).unwrap().start();
        let next_measure = re_measure.find_at(pvsc_content, measure_start + 1).map(|m| m.start()).unwrap_or(pvsc_content.len());
        let measure_block = &pvsc_content[measure_start..next_measure];
        for ev in re_event.captures_iter(measure_block) {
            let id = ev[1].parse::<usize>().unwrap_or(0);
            let numer = ev[2].parse::<usize>().unwrap_or(1);
            let denom = ev[3].parse::<usize>().unwrap_or(1);
            duration_map.insert((current_measure, id), (numer, denom));
        }
    }

    // SVGパラメータ
    let width = 600;
    let height = 120;
    let staff_top = 40;
    let staff_left = 50;
    let staff_spacing = 12;
    let staff_lines = 5;
    let note_radius = 7;

    // 5線譜を描画
    let mut group = Group::new();
    for i in 0..staff_lines {
        let y = staff_top + i * staff_spacing;
        group = group.add(Line::new()
            .set("x1", staff_left)
            .set("y1", y)
            .set("x2", width - staff_left)
            .set("y2", y)
            .set("stroke", "black")
            .set("stroke-width", 2));
    }

    // YAMLに存在するid一覧を作成
    let yaml_ids: HashSet<_> = notes.iter().map(|n| n.id).collect();

    // 音符を等間隔で配置
    let note_count = notes.len().max(1);
    let note_spacing = ((width - staff_left * 2) as f32) / (note_count as f32 + 1.0);

    for (i, note) in notes.iter().enumerate() {
        let cx = staff_left as f32 + note_spacing * (i as f32 + 1.0);
        let cy = staff_top as f32 + staff_spacing as f32 * 2.0; // 仮の高さ（譜表中央）

        // duration合計計算
        let mut total_numer = 0;
        let mut total_denom = 1;
        // id: note.id のduration
        if let Some(&(numer, denom)) = duration_map.get(&(note.measure, note.id)) {
            total_numer = numer;
            total_denom = denom;
        }
        // 連番idでyamlに存在しないidのdurationを合算
        let mut next_id = note.id + 1;
        while !yaml_ids.contains(&next_id) {
            if let Some(&(numer, denom)) = duration_map.get(&(note.measure, next_id)) {
                // 分数加算: a/b + c/d = (a*d + c*b)/(b*d)
                total_numer = total_numer * denom + numer * total_denom;
                total_denom = total_denom * denom;
                // 約分
                let d = gcd(total_numer, total_denom);
                if d > 1 {
                    total_numer /= d;
                    total_denom /= d;
                }
            }
            next_id += 1;
            if next_id > note.id + 10 { break; }
        }

        // 合計durationが2/1, 4/2, 6/3など約分して2/1なら白丸
        let d = gcd(total_numer, total_denom);
        let (reduced_numer, reduced_denom) = if d > 1 {
            (total_numer / d, total_denom / d)
        } else {
            (total_numer, total_denom)
        };
        let fill = if reduced_numer == 2 && reduced_denom == 1 {
            "white"
        } else {
            "black"
        };

        let mut circle = Circle::new()
            .set("cx", cx)
            .set("cy", cy)
            .set("r", note_radius)
            .set("stroke", "black");
        if fill == "white" {
            circle = circle.set("fill", "white");
        } else {
            circle = circle.set("fill", "black");
        }
        group = group.add(circle);
    }

    let document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .add(group);

    svg::save(output_path, &document)?;
    Ok(())
}
