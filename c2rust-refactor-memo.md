# C world Based Refactoring to Rust

This approach is supposed you have a code-base written in C language and you kick-offed from one C-function to change all the C-world.

You will attempt to find all tools could fit and automate your actions.

## 1 makefile remained when Rust introduced

If there is no unit test, just add it and put into makefile to be automated.
Let `make test` work as before.

## 2 Add unit test code written in Rust which call C-written functions

The best wish was that we have one tool can generate Rust-functions for tests from C-functions.
For this case, cntlm has no unit tests. I must create all unit tests in Rust and export them to cntlm's C world where I need a tool that automate the exporting.

rust-bindgen

### 2.1 Choose or create C Header for the C-function you will test

我选择config部分作为开始, 用bindgen通过config.h生成其rust接口以便后面用rust的测试代码调用 
当然中间用到了bindgen的优秀实践[build.rs](https://rust-lang.github.io/rust-bindgen/tutorial-3.html)


### 2.2 How get started unit test in Rust World

[学习材料](https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html)



## 3 Implement these functions in Rust guarded by above test code, and make remaining C-written functions call it

## 4 Do step 2 & 3, until replaced all C-written  function
