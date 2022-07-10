pub fn function() {}

// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数：
fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}
