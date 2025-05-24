// CLIコマンド定義
use clap::{Parser, Subcommand, Args as ClapArgs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    Generate {
        #[arg(long)]
        input: String,
        #[arg(long)]
        output: String,
    },
    /// 楽譜をSVGでレンダリングする
    Render(RenderArgs),
}

#[derive(ClapArgs)]
pub struct RenderArgs {
    /// 出力SVGファイル名
    #[arg(long)]
    pub output: String,
}
