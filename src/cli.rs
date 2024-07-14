use clap::Parser;

use crate::language_type::LanguageType;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// 需要被计算代码行数的文件夹或者文件
    #[arg(default_value = ".")]
    pub name: String,

    /// 指定目标语言类型
    #[arg(value_enum, short, long, default_value = "cpp")]
    pub target: LanguageType,
}
