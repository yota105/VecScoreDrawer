//! データ型変換モジュール
//! vsc::parser の型 → data.rs の型 への変換をまとめる

use crate::parser;
use crate::data;

pub fn vsc_score_to_data_score(vsc_score: data::Score) -> data::Score {
    vsc_score
}
// もしmain.rsでvsc::parser::Score型を受け取っている場合は、
// そのままdata::Scoreとして扱ってOKです。
