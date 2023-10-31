# TESTING

[编写自动化测试](https://kaisery.github.io/trpl-zh-cn/ch11-00-testing.html)

### 常用的测试方法

```sh
# 显示函数输出
$ cargo test -- --show-output
```

```sh
# 通过指定名字来运行部分测试
$ cargo test [method_name]
```

```rust
// 忽略某些测试
#[ignore]
fn test_case() {}
```

```sh
# 运行被忽略的测试
$ cargo test -- --ignored
```

### 测试的组织架构

#### 单元测试
单元测试与它们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。

#### 集成测试
在 Rust 中，集成测试对于你需要测试的库来说完全是外部的。同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API。集成测试的目的是测试库的多个部分能否一起正常工作。一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也是很重要的。为了创建集成测试，你需要先创建一个 tests 目录。

运行 `tests` 目录中我们指定的文件 `integration_test.rs` 中的测试。
```sh
$ cargo test --test integration_test
```

#### 二进制 crate 的集成测试
如果项目是二进制 `crate` 并且只包含 `src/main.rs` 而没有 `src/lib.rs`，这样就不可能在 `tests` 目录创建集成测试并使用 `extern crate` 导入 src/main.rs 中定义的函数。只有库 `crate` 才会向其他 `crate` 暴露了可供调用和使用的函数；二进制 `crate` 只意在单独运行。