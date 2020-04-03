// 链接到 `rary` 库，导入其中的项
extern crate rary;

fn main() {
    rary::public_function();

    // 报错！`private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}

// rustc 11-02-link.rs -o executable --extern rary=library.rlib && ./executable
