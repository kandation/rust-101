// Rust-101, ส่วน 00: ประเภทตัวเลขผสม
// ======================================

// โค๊ตชิ้นแรกของเรา, เราจะมาเขียนฟังก์ชันที่คำนวณหาค่าต่ำสุดของรายการ

//@ ## มาเริ่มกันเถอะ!
//@มาเริ่มต้นด้วยการคิดเกี่ยวกับ *type* ของฟังก์ชันของเราก่อน. Rust กำหนดให้เราต้องระบุประเภทของอาร์กิวเมนต์ทั้งหมด 
//@ และ*type*ของค่าที่ส่งคืน, ก่อนที่เราจะเริ่มเขียนเนื้อหาได้ ในกรณีของฟังก์ชันค่าต่ำสุดของเรา
//@ ตอนนี้ เราอาจจะคิดว่าฟังก์ชันนี้ส่งคืนเป็นตัวเลข. แต่เราก็พบกับปัญหาบางอย่าง:
//@ แล้วอะไรคือค่าต่ำสุดของลิสต์ว่างกันล่ะ? ในการประกาศ type ของฟังก์ชั่นบอกเราว่า เราควรคืนค่า
//@ *บางสิ่ง* ออกไป. โดยทั่วไปเราก็จะคืนค่าเป็น 0, แต่มันช่างไร้เหตุผลเลยจริงไหม.
//@ เราต้องการ *type* ที่เป็นทั้ง "ตัวเลข หรือ ไม่มีอะไรเลย". Type ดังกล่าวเรียกว่า "ประเภทตัวเลขลูกผสม (algebraic datatype)" 
//@ หรือก็คือ ตัวเลือกหลายรายการ, และในรัสต์จะนิยามประเภทดังกล่าวด้วยคีย์เวิร์ด `enum`.
//@ สำหรับคนที่คุ้นเคยกับ C++ คุณสามารถคิดว่าประเภทดังกล่าวเป็น `union` พร้อมกับฟิลด์ที่เก็บตัวแปรของ union ที่กำลังใช้งานในปัจจุบัน.


// และ `enum` สำหรับ "ตัวเลข หรือ ไม่มีอะไรเลย" เขียนได้ดังนี้:
enum NumberOrNothing {
    Number(i32),
    Nothing
}
//@ Notice that `i32` is the type of (signed, 32-bit) integers. To write down the type of
//@ the minimum function, we need just one more ingredient: `Vec<i32>` is the type of
//@ (growable) arrays of numbers, and we will use that as our list type.

// Observe how in Rust, the return type comes *after* the arguments.
fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    //@ In the function, we first need some variable to store the minimum as computed so far.
    //@ Since we start out with nothing computed, this will again be a 
    //@ "number or nothing":
    let mut min = NumberOrNothing::Nothing;
    //@ We do not have to write a type next to `min`, Rust can figure that out automatically
    //@ (a bit like `auto` in C++11). Also notice the `mut`: In Rust, variables are
    //@ immutable per default, and you need to tell Rust if you want
    //@ to change a variable later.

    // Now we want to *iterate* over the list. Rust has some nice syntax for iterators:
    for el in vec {
        // So `el` is an element of the list. We need to update `min` accordingly, but how do we
        // get the current number in there? This is what pattern matching can do:
        match min {
            // In this case (*arm*) of the `match`, `min` is currently nothing, so let's just make
            // it the number `el`.
            NumberOrNothing::Nothing => {
                min = NumberOrNothing::Number(el);                  /*@*/
            },
            // In this arm, `min` is currently the number `n`, so let's compute the new minimum and
            // store it.
            //@ We will write the function `min_i32` just after we completed this one.
            NumberOrNothing::Number(n) => {
                let new_min = min_i32(n, el);                       /*@*/
                min = NumberOrNothing::Number(new_min);             /*@*/
            }
        }
        //@ Notice that Rust makes sure you did not forget to handle any case in your `match`. We
        //@ say that the pattern matching has to be *exhaustive*.
    }
    // Finally, we return the result of the computation.
    return min;
}

// Now that we reduced the problem to computing the minimum of two integers, let's do that.
fn min_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;                                                   /*@*/
    } else {
        return b;                                                   /*@*/
    }
}

// Phew. We wrote our first Rust function! But all this `NumberOrNothing::` is getting kind of
// ugly. Can't we do that nicer?

// Indeed, we can: The following line tells Rust to take
// the constructors of `NumberOrNothing` into the local namespace.
// Try moving that above the function, and removing all the occurrences of `NumberOrNothing::`.
use self::NumberOrNothing::{Number,Nothing};

// To call this function, we now just need a list. Of course, ultimately we want to ask the user for
// a list of numbers, but for now, let's just hard-code something.

//@ `vec!` is a *macro* (as indicated by `!`) that constructs a constant `Vec<_>` with the given
//@ elements.
fn read_vec() -> Vec<i32> {
    vec![18,5,7,1,9,27]                                             /*@*/
}

// Of course, we would also like to actually see the result of the computation, so we need to print the result.
//@ Of course Rust can print numbers, but after calling `vec_min`, we have a `NumberOrNothing`.
//@ So let's write a small helper function that prints such values.

//@ `println!` is again a macro, where the first argument is a *format string*. For
//@ now, you just need to know that `{}` is the placeholder for a value, and that Rust
//@ will check at compile-time that you supplied the right number of arguments.
fn print_number_or_nothing(n: NumberOrNothing) {
    match n {                                                       /*@*/
        Nothing => println!("The number is: <nothing>"),            /*@*/
        Number(n) => println!("The number is: {}", n),              /*@*/
    };                                                              /*@*/
}

// Putting it all together:
pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    print_number_or_nothing(min);
}

//@ You can now use `cargo build` to compile your *crate*. That's Rust's name for a *compilation unit*, which in
//@ the case of Rust means an application or a library. <br/>
// Finally, try `cargo run` on the console to run it.

//@ Yay, it said "1"! That's actually the right answer. Okay, we could have
//@ computed that ourselves, but that's beside the point. More importantly:
//@ You completed the first part of the course.

//@ [index](main.html) | previous | [raw source](workspace/src/part00.rs) | [next](part01.html)
