# 代码行数统计工具

实现了 python、ruby、cpp 的代码行数统计工具。

## 构建

### 环境配置

首先需要在电脑中安装 rust，对于国内网络，可以使用字节跳动提供的镜像。如果可以流畅访问外网，可以参考[入门 - Rust 程序设计语言](https://www.rust-lang.org/zh-CN/learn/get-started)。

步骤一：设置 Rustup 镜像， 修改配置 `~/.zshrc` 或者 `~/.bashrc`

```bash
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

步骤二：安装 Rust（请先完成步骤一的环境变量导入并 source rc 文件或重启终端生效）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
```

之后全选默认即可。

步骤三：设置 crates.io 镜像， 修改配置 `~/.cargo/config`，已支持 git 协议和 sparse 协议，>=1.68 版本建议使用 sparse-index，速度更快。

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
```

运行以下命令检查是否成功安装

```bash
cargo --version
```

### 编译代码

若已经成功安装，在当前目录下运行下述命令

```bash
cargo install --path . --locked
```

即可成功安装命令行工具`cloc`

如果未能成功安装，运行`cargo b --release`之后在当前目录的 `target/release` 目录下找到 cloc 二进制文件即可。

## 使用

运行`cloc --help`即可查看帮助信息。

```bash
$ cloc --help
Usage: cloc [OPTIONS] [NAME]

Arguments:
  [NAME]  需要被计算代码行数的文件夹或者文件 [default: .]

Options:
  -t, --target <TARGET>  指定目标语言类型 [default: cpp] [possible values: cpp, ruby, python]
  -h, --help             Print help
  -V, --version          Print version
```

默认递归的统计当前目录下的 cpp 代码，可以设置 -t 参数指定目标语言类型，参数为指定目录或者文件。

可以更改 languages.json 文件来添加新的语言类型。

## 设计

### 总体设计

- 通过迭代器模式遍历文件夹或者文件。
- 通过 rust 的元编程能力，从 languages.json 中读取内容，生成对应的信息。
- 通过状态机，判断当前代码所处的状态，同时标记当前行的类型。

### 目录结构

代码目录结构如下：

```
cloc
├── build.rs               编译前执行的内容，用于生成代码
├── Cargo.lock
├── Cargo.toml             rust项目配置文件
├── gen                    用于根据 languages.json 预生成代码
│   ├── gen_code.rs        用于生成代码
│   ├── language.rs        用于描述json结构体以便反序列化
│   └── mod.rs             模块入口
├── languages.json         语言配置文件
├── README.md
└── src
    ├── cli.rs             命令行参数配置
    ├── counter.rs         统计代码行数
    ├── language_type.rs   语言类型
    ├── main.rs            程序入口
    ├── state.rs           描述当前行状态以及类别
    ├── table.rs           用于表格输出
    └── walker.rs          用于遍历文件
```

## 参考

实现过程中参考了 tokei，同时使用一些代码统计工具和此项目作对比。

对于 [cloc](https://github.com/AlDanial/cloc)

- 原始字符串中的注释会被识别为注释，而不是代码。

对于 [tokei](https://github.com/XAMPPRocky/tokei)

- 如果多行注释位于开头，则后面即使有代码也会被识别为注释。
- 注释中的空行会被识别为注释。

对于 [loc](https://github.com/cgag/loc)，

- 原始字符串中的注释会被识别为注释，而不是代码。
- 字符串中的注释会被识别为注释。
- 字符串中有多行注释开头而没有结尾，会将后续内容全部识别为注释。
