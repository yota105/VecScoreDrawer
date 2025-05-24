// CLIコマンド定義
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "VecScoreDrawer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the score to standard output or file
    Print {
        input: String,
    },
    /// sample.vsc→score_workspace/score_def/score_def.yamlを生成
    GenerateScore,
}
