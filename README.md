# play-ground

## 工程结构与配置

### Packages and Crates

第一部分是`packages`和`crates`.一个`crate`就是一个人可执行二进制文件或者是一个库.
我们将Rust编译时所使用的入口文件称作这个单元包的根节点,它同时也是单元包的根模块.而`packages`则由一个或者多个提供相关功能的`crates`
集合而成,它所附带的配置文件`Cargo.toml`描述了如何编译这些`crates`.

当我们创建了一个`package`的时候都发生了什么.使用命令`cargo new my-project`会创建以下目录结构

```bash
my-project/
├── Cargo.toml
└── src
    └── main.rs
```
查看`Cargo.toml`文件的内容,我们会发现没有提到`src/main.rs`,因为`Cargo`遵守了一个惯例:`src/main.rs`是二进制`crate`的根节点,这个`crate`与`package`同名.
同样,如果存在`src/lib.rs`,那么这个`package`包含了一个与`package`同名的库`crate`,`src/lib.rs`这个`crate`的根节点.
`Cargo`将`crate`根节点文件传到`rustc`用于编译库和二进制执行文件.


`src/main.rs`
`src/lib.rs`
`src/bin`