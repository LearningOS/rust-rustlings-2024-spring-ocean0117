// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a = "独立寒秋，湘江北去，橘子洲头。

             看万山红遍，层林尽染；漫江碧透，百舸争流。

             鹰击长空，鱼翔浅底，万类霜天竞自由。

             怅寥廓，问苍茫大地，谁主沉浮？

             携来百侣曾游，忆往昔峥嵘岁月稠。

             恰同学少年，风华正茂；书生意气，挥斥方遒。

             指点江山，激扬文字，粪土当年万户侯。

             曾记否，到中流击水，浪遏飞舟！";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
