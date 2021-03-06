fn main() {
    // 通过闭包和函数实现增量。
    fn  function            (i: i32) -> i32 { i + 1 }

    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住代码都是可选的。
    // 这些匿名函数（nameless function）赋值给合适命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());

    // 从封闭的环境中捕获变量是可能的；而这行为对函数来说是不可能的。
    let professor_x = "Charles Xavier";

    // 下面闭包从封闭的环境中打印一个变量，没有参数和返回值。
    let print = || println!("Professor X's name is: {}", professor_x);

    // 调用闭包。
    print();
}
