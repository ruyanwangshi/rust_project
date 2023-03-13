fn main() {
    // 实现一个通用逻辑门
    // NAND和NOR门被称为通用逻辑门，使用这俩逻辑门来实现任何逻辑电路；
    // 尽管可以使用三个基本们（与门，或门，非门）来实现任何复杂度的逻辑电路，
    // 但也可以使用一种门来实现它们，因此有时将“与非”门或“非”门为通用建筑“块”。
    // 因此，AOI逻辑可以被转换成NAND或NOR逻辑。
    // 对应的复习链接
    // https://blog.csdn.net/cumt30111/article/details/107793892

    // 或非门
    {
        // 或非门也是俩个门的组合，即“或”门，其后是“非”门，其中使用“非”门将“或”门的输出
        // 反向以获得最终输出，或非门的逻辑运算可以写成Y = A+B

        // 或门
        fn OR(a: bool, b: bool) -> bool {
            return a || b;
        }

        // 非门
        fn NOT(value: bool) -> bool {
            return !value;
        }

        // 与门
        fn AND(a: bool, b: bool) -> bool {
            return a && b;
        }

        // 或非门
        fn NOR(a: bool, b: bool) {
            let a = OR(a, b);
            let b = NOT(a);
            println!("{}", b);
            // return OR(a) + NOT(b)
        }

        NOR(false, false);
        NOR(true, false);
        NOR(false, true);
        NOR(true, true);

        println!("分隔线~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

        // 与非门
        fn NAND(a: bool, b: bool) {
            let a = AND(a, b);
            let b = NOT(a);

            // let a = a && b;
            // let b = 

            println!("{}", b);
        }

        NAND(false,false);
        NAND(true,false);
        NAND(false,true);
        NAND(true,true);
        
    }


    println!("Hello, world!");
}
