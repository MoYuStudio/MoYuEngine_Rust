# MoYuEngine_Rust

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000)
![License: Apache 2.0](https://img.shields.io/badge/License-blue.svg)

MoYuEngine_Rust是一个由Rust编程语言编写的游戏引擎。本引擎主要用于创建2D和3D游戏，而且重点在于高效、快速、安全和易于使用。由MoYuStudio研发。

## 特性

- **性能**: MoYuEngine_Rust使用了Rust语言的强大功能，提供了优秀的性能和内存安全性。
- **跨平台**: MoYuEngine_Rust可以在多种操作系统和硬件上运行，包括但不限于Windows、MacOS、Linux、Android和iOS。
- **2D/3D图形渲染**: 具有先进的2D和3D渲染能力，支持多种图形效果和优化。
- **物理模拟**: 提供了内建的物理模拟系统，用于创建真实的游戏体验。
- **音频处理**: 支持游戏中的音频处理，包括音效和背景音乐。

## 安装

在您的Rust项目中添加如下内容到您的Cargo.toml文件:

```rust
[dependencies]
moyu_engine = "1.0.0"

然后在你的项目中，添加如下的import声明： 

生锈 

extern crate moyu_engine;

##使用说明 

使用MoYuEngine_Rust创建游 

生锈 

use moyu_engine::{Engine, Game};

struct MyGame;

impl Game for MyGame {
    // 编写您的游戏逻辑
}

fn main() {
    let mut engine = Engine::new();
    engine.run(MyGame);
}

## 贡献 

我们欢迎一切形式的贡献。您可以通过提交问题、提出新功能的建议、改进代码或者改进文档来贡献我们的项目。 
许可证 

本项目采用 Apache 2.0 许可证，详细信息请参见 LICENSE 。 
联系方式 

如有任何疑问或需要帮助，欢迎联系我们:  cprewilsonvinson@gmail.com 

## 致谢 

感谢所有对MoYuEngine_Rust项目做出贡献的人。 

我们期待你的参与，共同打造一个更好的游戏引擎！ 