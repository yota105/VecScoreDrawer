pub fn process_ties(elements: Vec<ScoreElement>) -> Vec<ScoreElement> {
    let mut out = Vec::new();
    for se in elements {
        match se {
            ScoreElement::Tie => {
                // 前の要素が Event であることを保証しつつ…
                if let Some(ScoreElement::Event(prev_ev)) = out.pop() {
                    let mut ev = prev_ev.clone();
                    ev.tie = true;
                    ev.duration += 1.0;  // 基本ユニットを加算
                    out.push(ScoreElement::Event(ev));
                } else {
                    // エラー扱いでも良い
                }
            }
            other => out.push(other),
        }
    }
    out
}
