# Yar

正在尝试第一次用Yew写一个完整的前端项目，拟作为一个web记账工具，满足日常的消费或者收入的个人账本，支持离线使用，也可选择数据同步到某个服务器。顺带一提Yar这个名字是“You are rich”的简写，属于是自我催眠。

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk].

### 安装

这是一个rust项目，需要先安装rust工具链，推荐使用rust官方工具链管理工具[`rustup`](https://www.rust-lang.org/tools/install)，已安装跳过这一步。

编译rust到WASM需要安装`wasm32-unknown-unknown`构建目标。已安装跳过这一步。

```bash
rustup target add wasm32-unknown-unknown
```

安装两个命令行工具：

```bash
cargo install trunk wasm-bindgen-cli
```

[trunk] 是rust web打包工具

然后环境就准备好了👌

### 运行

```bash
trunk serve
```

会启动一个http服务供你访问，并且在代码更新时会自动刷新页面

### 编译

```bash
trunk build
```

加 `--release` flag编译为发布状态，默认编译结果输出到 `dist` 文件夹

### License

MIT

[trunk]: https://github.com/thedodd/trunk
