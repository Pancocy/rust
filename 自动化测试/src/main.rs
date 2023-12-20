/* Rust 提供的专门用来编写测试的功能：test 属性、一些宏和 should_panic 属性。

*/


fn main() {
    
}

/*
    在Rust中，测试时一个复杂的概念：通常要关注两类测试 ： 单元测试 和 集成测试。
        1.单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。
        测试模块的 #[cfg(test)] 标注告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，而在运行 cargo build 时不这么做。
        2.集成测试
 */

#[cfg(test)]
mod test{
    #[test]
    fn it_works(){

    }

}

