# score モジュール

楽譜（意味層）のデータ型・パーサ・レイアウト情報を管理します。
- model.rs: 楽譜のデータ型
- parser.rs: 楽譜定義ファイルのパーサ

仮表示まで可能にします。(htmlみたいなもの)

全体の行数が非常に長くなるため、
・全体の定義
・楽器ごとに10小節ごと
に分割します。

分割機能は後で実装するため、しばらくは一つのファイルで開発を行います。

## スコア定義オプションのリスト



score: スコアの単位を指します。scoreは以下のプロパティを持ちます: tempo, key_signature, parts
  tempo: 楽曲のテンポです。未設定の場合、デフォルト値の120になります。tempoは以下のプロパティを持ちます: measure, position, strict_position, bpm, tempo_mark
    measure (必須, i32, 1以上): テンポを変更する小節番号です。tempoがmeasure: 1における値を持たない場合とVecScoreに記された範囲外の値を指定した場合、エラーとなります。
    position (必須, f32, 1.0~999.999...): テンポを変更する位置です。何かしらのScoreElementの位置（分数で表現）と完全に一致しない場合、一番近いScoreElementの位置に設定されます。何かしらのScoreElementの位置（分数で表現）と十分に近い値でない場合、警告が出ます。また、VecScoreの当該measureにおける範囲外の場合、エラーとなります。
    strict_position(bool, デフォルトはfalse): trueの場合、何かしらのScoreElementと近い位置にある場合でも、絶対にその位置に設定します。
    bpm (tempo_markとどちらか必須, f32, 0.0~511.999...、デフォルトは120): 楽曲のbpmです。tempo_markと同時に設定することもできます。
    tempo_mark (bpmとどちらか必須, 文字列): ユーザー定義のTempoMark型に一致する場合はbpmが自動設定されます。一致しない場合はデフォルト値となります。bpmと同時指定時はbpmが優先されます。(後でテンポ変化実装)

  key_signature: 調号を示します。未設定の場合、measure: 1, position: 1.0, noneになります。key_signatureは以下のプロパティを持ちます: measure, position, key
    measure (必須, i32, 1以上): 調号を変更する小節番号です。key_signatureがmeasure: 1における値を持たない場合とVecScoreに記された範囲外の値を指定した場合、エラーとなります。
    position (必須, f32, 1.0~999.999...): テンポを変更する位置です。何かしらのScoreElementの位置（分数で表現）と完全に一致しない場合、一番近いScoreElementの位置に設定されます。また、VecScoreの当該measureにおける範囲外の場合、エラーとなります。
    key (必須, keyType型または[noneまたはflatまたはsharp, 1~7]): ユーザー定義のkey型、またはシャープかフラットの数を指定する方式に一致しない場合、エラーになります。(後程カスタムkeyの設定方法を考える)

  parts (必須): パートを示します。partsは以下のプロパティを持ちます: name, instrument_change, transposition, unique_key, key_signature, staves, dynamics, notes
    name (必須、文字列): ユーザー定義のinstrument型に一致する場合はtranspositionを自動設定します。VecScoreのパート名と同一である必要があります。
    instrument_change (文字列): 楽器の変更を示します。ユーザー定義のtempo_mark型に一致する場合はtranspositionを自動設定します。instrument_changeは以下のプロパティを持ちます: measure, position, transposition_intaval
      measure (必須, i32, 1以上): 楽器を変更する小節番号です。instrument_changeがmeasure: 1における値を持たない場合とVecScoreに記された範囲外の値を指定した場合、エラーとなります。
      position (必須, f32, 1.0~999.999...): 楽器を変更する位置です。何かしらのScoreElementの位置（分数で表現）と完全に一致しない場合、一番近いScoreElementの位置に設定されます。何かしらのScoreElementの位置（分数で表現）と十分に近い値でない場合、警告が出ます。また、VecScoreの当該measureにおける範囲外の場合、エラーとなります。
      transposition_intaval: partsのtransposition_intavalと同様。

    transposition: instrument型に一致していても、transpositionが設定されている場合、こちらが優先されます。デフォルトはそれぞれ1, 1, 0が設定されています。気を付けるべきこととして、transpositionが変更された場合、見かけの調号も変更されることになります。transpositionは以下のプロパティを持ちます: measure, position, intaval
      measure (必須, i32, 1以上): 楽器を変更する小節番号です。transpositionがmeasure: 1における値を持たない場合とVecScoreに記された範囲外の値を指定した場合、エラーとなります。
      position (必須, f32, 1.0~999.999...): 楽器を変更する位置です。何かしらのScoreElementの位置（分数で表現）と完全に一致しない場合、一番近いScoreElementの位置に設定されます。何かしらのScoreElementの位置（分数で表現）と十分に近い値でない場合、警告が出ます。また、VecScoreの当該measureにおける範囲外の場合、エラーとなります。
      transposition_intaval (必須, i32): 通常のinCの楽器と鳴る音が半音いくつ分ずれているか示します。例: Bbクラリネットの場合、-2
    unique_key (bool, デフォルトはfalse): trueの場合、そのパートは全体のkey設定の影響を受けなくなります。
    key_signature (unique_keyがtrueの場合必須、keyType型または[noneまたはflatまたはsharp, 1~7]): score全体のkey_signatureと同様。unique_keyがfalseの時に設定するとエラーになる。(現状そうするが、trueのときの挙動についてはいろいろ考える)
    staves (partやinstrumentから自動設定): 線の数や段数、音部記号などを決める。自動設定されていた場合でも、stavesが設定されていた場合、こちらを優先します。stavesは以下のプロパティを持ちます: measure, position, type, staff_count, clef, lines, chromatic_assignment
      measure (必須, i32, 1以上): 線の数や段数を変更する小節番号です。transpositionがmeasure: 1における値を持たない場合とVecScoreに記された範囲外の値を指定した場合、エラーとなります。
      position (必須, f32, 1.0~999.999...): 線の数や段数を変更する位置です。何かしらのScoreElementの位置（分数で表現）と完全に一致しない場合、一番近いScoreElementの位置に設定されます。何かしらのScoreElementの位置（分数で表現）と十分に近い値でない場合、警告が出ます。また、VecScoreの当該measureにおける範囲外の場合、エラーとなります。
      type (staff_countとどちらか必須、grandかsingleのみ): 一段譜か大譜表です。
      staff_count (typeとどちらか必須、1~8): 段数です。
      clef (singleの場合partやinstrumentから自動設定、grandの場合自動的に[Treble, Bass]、clef型、要素の数をtypeやstaff_countに一致): 音部記号。staffが複数の場合、[Treble, Bass]のように表します。要素の数がstaffやtypeと合わない場合、エラーとなります。
      lines (1~20, デフォルトでは全ての要素が5): 一線譜から五線譜、そしてそれ以上を設定できます。10以上は見ずらいので推奨されません。staffが複数の場合、[5, 5]のように表します。要素の数がstaffやtypeと合わない場合、エラーとなります。
      chromatic_assignment (bool, デフォルトはfalseだが、percussionの場合のみデフォルトでtrue): trueの場合、全音階ベースではなく、半音階ベースで位置を割り当てます。特に打楽器において有効です。
    dynamics: dynamicsは以下のプロパティを持ちます: measure, position, level, change, change_mode, text
      measure (必須, i32, 1以上): dynamicsを変更する小節番号です。transpositionがmeasure: 1における値を持たない場合とVecScoreに記された範囲外の値を指定した場合、エラーとなります。
      position (必須, f32, 1.0~999.999...): dynamicsを変更する位置です。何かしらのScoreElementの位置（分数で表現）と完全に一致しない場合、一番近いScoreElementの位置に設定されます。何かしらのScoreElementの位置（分数で表現）と十分に近い値でない場合、警告が出ます。また、VecScoreの当該measureにおける範囲外の場合、エラーとなります。
      level (文字列、一致する場合DynamicsLevel型): 変化するダイナミクスです。
      change (文字列、一致する場合DynamicChange型): 同じ位置の場合はlevelの後ろにつきます。後で実装 (pocoとかめんどくさい)
      change_mode (changeが値を持つ場合必須, letterかsymbol): DynamicChangeがCrescendo, Diminuendo, Cresc, Dim, Decrescendoの場合、symbolが有効。symbolにも関わらずこれらでない場合、エラー。
      change_end (change_modeがsymbolの場合必須) change_endは以下のプロパティを持ちます。
        measure (必須, i32, 1以上): 記号の終端となる位置の小節番号です。値が上位のdynamics未満の場合とVecScoreに記された範囲外の値を指定した場合、エラーとなります。
        position (必須, f32, 1.0~999.999...): 記号の終端となる位置です。何かしらのScoreElementの位置（分数で表現）と完全に一致しない場合、一番近いScoreElementの位置に設定されます。何かしらのScoreElementの位置（分数で表現）と十分に近い値でない場合、警告が出ます。また、change_endのmeasureがdynamicsのmeasureと同じかつchange_endのpositionがdynamicsのposition以下の場合と、VecScoreの当該measureにおける範囲外の場合、エラーとなります。
      text (文字列): 補足テキスト、同じ位置の場合はlevelやchangeの後ろにくっつきます。(subitoとか前にくっつくやつは後で実装)
    notes: VecScore、outputのVecScore(一時ファイル、名前を考える)の変更をリアルタイムで監視します。初回読み込み時、全てのNote、Chordの構成音、Tieについて、対応するIDと、デフォルト値のaccidentals, articulationsを生成します。notesは以下のプロパティを持ちます: measure, id, attributes
      measure (必須, i32, 1以上): noteを特定するための小節番号です。VecScoreに記された範囲外の値を指定した場合、エラーとなります。
      id (必須, i32, 1以上): vscパーサにより自動生成されたidです。出力結果に含まれないidを指定した場合、エラーとなります。chordの構成音のidはarticulationが必ずnoneになり、それ以外の場合はエラーになります。chordにおける実際のアーティキュレーションは、chord自体のidのものが適用されます。
      attributes (必須): note, chord, tieの持つ属性です。attributesは以下のプロパティを持ちます: scale_division, accidental, articulations, slur
        scale_division (ScaleDivision型, デフォルトは12): (範囲指定コマンドで一括変更可能にする。)ScaleDivision型以外の入力があった場合はエラーとなります。
        accidental (必須, Accidental型) :音名による入力の場合、accidentalはそこから自動決定されます。MIDI note numberの場合、別のロジックにより自動で決定されます。臨時記号の内容と音高が一致しない場合エラーとなります。(あとで実装: scale_divisionに基づき、pitch_centsから最も近い値が設定されます。)
        articulations (必須, Articulation型 例: [staccato, tenuto]): 重複可能なオプションあり。重複不可能な場合(同一のもの、noneを含む場合など)はエラーとなります。
        notehead (bool, デフォルトはtrue): falseの場合、譜頭を省略します。
        slur (bool, デフォルトはfalse): trueだった場合、スラーを配置します。slurがtrueの音が始点となります。
        slur_end_measure (slurがtrueのとき必須, i32, 1以上): スラーの終端の音符が含まれるmeasureです。始点より小さい値であった場合、エラーとなります。
        slur_end_id (slurがtrueのとき必須, i32, 1以上): スラーの終端のidです。chordの場合は、chordのidを指定します。始点より前の音だった場合、エラーとなります。
        trill (bool)
        pitch_slide: ピッチを滑らかに変化させます。記入がない場合は追加されません。pitch_slideは以下のプロパティを持ちます: type, 
          text (bool, デフォルトはtrue): 線の隣に文字を追加するか決定します。
          type (GlissandoかPortamento, デフォルトはglissando): textがtrueの場合、どちらかから選択します。textがfalseの場合に設定するとエラーとなります。
          connection (bool, デフォルトはtrue): 線の先に音符が配置されているか決めます。trueの場合、その音の譜頭とslide_end_idで設定された譜頭の間に線が繋がれます。その間がnoteあるいは始点のchordと同数の構成音によるchordで埋まっている場合、譜頭は省略され、結ばれた2つの譜頭のみが残ります。falseかつslide_end_measureおおよび
          slide_end_measure (slide_end_idまたはslide_end_pitchを持つ場合必須, i32, 1以上): 結ばれる終点の譜頭を持つ音符のあるmeasureを示します。
          slide_end_id (slide_end_measureを持つ場合、slide_end_positionとどちらか必須、i32, 1以上): 結ばれる終点の譜頭を持つ音符のidを示します。noteからchord, chordからnote、構成音数の異なるchord間、restとの間は結ぶことができず、エラーになります。また、slide_end_positionと同時に設定した場合、エラーとなります。
          slide_end_position (slide_end_measureを持つ場合、slide_end_idとどちらか必須, f32, 1.0~999.999...): idで指定できない、例えば音符に向かわないなどの場合は、postionにより指定します。slide_end_idと同時に設定した場合、エラーとなります。
          slide_end_note (slide_end_positionを指定した場合必須, midi_note_numberまたはnote_name): どの高さに線の終端を向かわせるかを決定します。slide_end_idと同時に設定した場合、slide_end_noteが優先されます。
          
  歌詞はtrueの場合、歌詞ファイルを参照 (後で実装)

```rs

enum TempoMark{
  Grave,      Largo,
  Larghetto,  Lento,
  Adagio,     Andante,
  Maestoso,   Andantino,
  Moderato,   Allegro_moderato,
  Animato,    Allegretto,
  Allegro,    Vivo,
  Assai,      Vivace,
  Presto,     Prestissimo
} // 今後追加予定、テンポと結び付ける



enum Key {
    Named(KeyType), // 例: C_Major, G_Minor など
    Pattern(KeyPattern), // 例: [sharp, 3] など
}

enum KeyType{
  none, // 無調あるいは調号無し
  C_Major,        A_Minor,        // 調号無し
  G_Major,        E_Minor,        // シャープ1つ
  D_Major,        B_Minor,        // シャープ2つ
  A_Major,        F_Sharp_Minor,  // シャープ3つ
  E_Major,        C_Sharp_Minor,  // シャープ4つ
  B_Major,        G_Sharp_Minor,  // シャープ5つ
  F_Sharp_Major,  D_Sharp_Minor,  // シャープ6つ
  G_Sharp_Major,  A_Sharp_Minor,  // シャープ7つ
  F_Major,        D_Minor,        // フラット1つ
  B_Flat_Major,   G_Minor,        // フラット2つ
  E_Flat_Major,   C_Minor,        // フラット3つ
  A_Flat_Major,   F_Minor,        // フラット4つ
  D_Flat_Major,   B_Flat_Minor,   // フラット5つ
  G_Flat_Major,   E_Flat_Minor,   // フラット6つ
  C_Flat_Major,   A_Flat_Minor,   // フラット7つ
}

struct KeyPattern(pub String, pub u8); // 例: ("sharp", 3) や ("flat", 2)


enum Clef {
  Treble,             // ト音記号（G clef, 通常は第2線）
  Bass,               // ヘ音記号（F clef, 通常は第4線）
  Alto,               // アルト記号（C clef, 第3線）
  Tenor,              // テノール記号（C clef, 第4線）
  Soprano,            // ソプラノ記号（C clef, 第1線）
  MezzoSoprano,       // メゾソプラノ記号（C clef, 第2線）
  BaritoneC,          // バリトン記号（C clef, 第5線）
  BaritoneF,          // バリトン記号（F clef, 第3線）
  SubBass,            // サブヘ音記号（F clef, 第5線）
  French,             // フレンチ記号（G clef, 第1線）
  Percussion,         // 打楽器記号
  None,               // 記号なし

  // オクターブ付き
  Treble8va,          // ト音記号＋8va（1オクターブ上）
  Treble8vb,          // ト音記号＋8vb（1オクターブ下）
  Treble15ma,         // ト音記号＋15ma（2オクターブ上）
  Treble15mb,         // ト音記号＋15mb（2オクターブ下）
  Bass8va,            // ヘ音記号＋8va（1オクターブ上）
  Bass8vb,            // ヘ音記号＋8vb（1オクターブ下）
  Bass15ma,           // ヘ音記号＋15ma（2オクターブ上）
  Bass15mb,           // ヘ音記号＋15mb（2オクターブ下）
}


enum DynamicsLevel{
  PPP,
  PP,
  P,
  MP,
  MF,
  F,
  FF,
  FFF,
  SF,
  SFZ,
  SFFZ,
  RFZ,
  SFP,
  SFPP,
  SFMP,
  FP,
  FPP,
  FMP,
  MFP,
  MFPP,
  MFMP,
  FFP,
  FFPP,
  FFMP,
}

enum DynamicsChange{

}

enum Accidental{
  // 通常臨時記号
  None,         Natural,
  Sharp,        Flat,
  DoubleSharp,  DoubleFlat,
  NaturalSharp, NaturalFlat,
  // 四分音
  QuarterSharp,      QuarterFlat,       // 1/4
  ThreeQuarterSharp, ThreeQuarterFlat,  // 3/4
  // 六分音以降は今度実装
}

enum Articulation {
  None,
  Staccato, Tenuto, Accent, Marcato, Fermata, BowUp, BowDown, Trill,
}

```

## レイアウト定義ファイル例

`sample_score_dif.yaml` は、YAML形式で楽譜のレイアウトや内容を定義するサンプルです。  
テンポ変更、パート構成、譜表の種類、音符の臨時記号やアーティキュレーションなどを柔軟に記述できます。
sample.vscに対応します。

```yaml
score:
  tempo:
    - measure: 1
      position: 1.0
      bpm: 120

  key_signature: 
    - measure: 1
      position: 1
      key: "C_Major"

  parts:
    - name: "Piano"
      staves:
        - measure: 1
          position: 1
          type: "grand" #またはstaff_count: 2, clef: ["treble", "bass"]
          lines: [5, 5] #未設定でも同様

      dynamics:
        - measure: 1
          position: 1
          level: "P"  #"p"でも可能にする

      notes: #以下は絶対自動生成できるようにする、特にタイを自動でまとめるロジック　小節ごとに分けると分かりやすい

        #------------<Measure 1>------------#
        - measure: 1
          id: 1
          attributes: 
            - accidental: "None"
              slur: "true"
              slur_end_measure: 1
              slur_end_id: 3
        - measure: 1
          id: 3
          attributes:
            - accidental: "None"
        - measure: 1
          id: 4
          attributes:
            - accidental: "None"

        #------------<Measure 2>------------#
        - measure: 2
          id: 1
          attributes: 
            - accidental: "None"
              slur: "true"
              slur_end_measure: 2
              slur_end_id: 5
        - measure: 2
          id: 2
          attributes: 
            - accidental: "None"
        - measure: 2
          id: 3
          attributes: 
            - accidental: "None"
        - measure: 2
          id: 4
          attributes: 
            - accidental: "None"
        - measure: 2
          id: 5
          attributes: 
            - accidental: "None"

        #------------<Measure 3>------------#
        - measure: 1
          id: 1
          attributes: 
            - accidental: "None"
              slur: "true"
              slur_end_measure: 3
              slur_end_id: 3
        - measure: 1
          id: 3
          attributes:
            - accidental: "None"
        - measure: 1
          id: 4
          attributes:
            - accidental: "None"

        #------------<Measure 4>------------#
        - measure: 2
          id: 1
          attributes: 
            - accidental: "None"
              slur: "true"
              slur_end_measure: 4
              slur_end_id: 5
        - measure: 2
          id: 2
          attributes: 
            - accidental: "None"
        - measure: 2
          id: 3
          attributes: 
            - accidental: "None"
        - measure: 2
          id: 4
          attributes: 
            - accidental: "None"
        - measure: 2
          id: 5
          attributes: 
            - accidental: "None"

```

