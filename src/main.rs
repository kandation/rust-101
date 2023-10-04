// ขอต้อนรับสู่ Rust-101
// ===================
// 
// นี่คือ [Rust-101](https://www.ralfj.de/projects/rust-101/), บทช่วยสอนขนาดเล็กสำหรับ
// [ภาษารัสต์](https://www.rust-lang.org/). มันถูกออกแบบมาให้เป็นหลักสูตรแบบโต้ตอบ,
// และปฏิบัติจริง: ฉันเชื่อว่าวิธีเดียวที่จะเรียนรู้ภาษาได้อย่าง *แท้จริง* คือการเขียนโค้ดในนั้น
// ดังนั้นคุณควรเขียนโค้ดระหว่างการเรียนรู้.
// 
// หากคุณมีคำถามที่ไม่ได้รับคำตอบที่นี่ โปรดดูที่ "แหล่งข้อมูลเพิ่มเติม" ด้านล่าง
// โดยเฉพาะอย่างยิ่ง แชนเนล IRC เต็มไปด้วยคนเก่งๆ ที่เต็มใจช่วยเหลือคุณ! 
// ฉันใช้เวลาอยู่ที่นั่นบ่อยมากเลยนะ ;-)
// 
// ฉันจะถือว่าคุณมีความคุ้นเคยกับการเขียนโปรแกรมในระดับหนึ่งแล้ว และจะไม่อธิบายแนวคิดพื้นฐาน
// ที่พบได้ทั่วไปในภาษาส่วนใหญ่ แต่จะมุ่งเน้นไปที่สิ่งที่ทำให้ Rust พิเศษแทน.
//
// ทำไมต้อง Rust?
// ---------
// 
// ตอนนี้ฉันขอสมมติว่าคุณได้ตัดสินใจที่จะลองเรียน Rust แล้ว ดังนั้นฉันไม่ต้องโน้มน้าวคุณมากนัก 
// แต่ในกรณีที่คุณยังไม่แน่ใจ ฉันจะมาเล่าให้ฟังว่าทำไมฉันคิดว่า Rust คุ้มค่าแก่การเรียนรู้:<br/>
// Rust เป็นภาษาที่มีเป้าหมายเฉพาะตัวมาก ณ เวลานี้ Rust มุ่งหวังที่จะให้การควบคุมหน่วยความจำ
// และพฤติกรรมการทำงานในระดับเดียวกันกับ C++  (เช่นการส่งข้อมูลไปมา (dispatch) ทั้งแบบคงที่และแบบไดนามิก)
// ซึ่งทำให้สามารถสร้าง abstractions ที่ไม่มีต้นทุนรันไทม์ได้ นอกจากนี้ 
// Rust ยังมีข้อดีของภาษาฟังก์ชันระดับสูงและความปลอดภัยที่รับประกันได้ (เช่น โปรแกรมจะไม่ล่มในลักษณะที่ควบคุมไม่ได้)
// ภาษาส่วนใหญ่ที่ใช้กันอยู่ในปัจจุบันจะต้องแลกเปลี่ยนการควบคุมกับความปลอดภัย (เช่น โดยบังคับใช้การใช้ garbage
//  collector) หรือ ในทางกลับกัน Rust สามารถทำงานได้โดยไม่ต้องมีการจัดสรรแบบไดนามิก
// (กล่าวคือ ไม่มี heap) และแม้กระทั่งไม่ต้องมีระบบปฏิบัติการ.
// Rust ยังสามารถป้องกันข้อผิดพลาดได้มากขึ้นกว่าภาษาที่ใช้ garbage collector  เพื่อความปลอดภัย นอกเหนือ
// จาก dangling pointers และ double-free แล้ว Rust ยังป้องกันปัญหาต่างๆ เช่น iterator invalidation
// และ data races สุดท้าย Rust จะทำความสะอาดให้คุณ และคืนทรัพยากร (หน่วยความจำ แต่ยังรวมถึง 
// file descriptors และอื่นๆ) เมื่อคุณไม่ต้องการใช้มันอีกต่อไป
// 
// 
// เริ่มต้นกันเลย!
// ---------------
// 
// ก่อนอื่น คุณต้องติดตั้ง Rust ซึ่งสามารถดาวน์โหลดได้จาก [เว็บไซต์ Rust](https://www.rust-lang.org/).
// ตรวจสอบให้แน่ใจว่าคุณติดตั้งเวอร์ชัน 1.3 หรือสูงกว่า คำแนะนำในการติดตั้งโดยละเอียดมีอยู่ใน
// [หนังสือเล่มนี้](https://doc.rust-lang.org/stable/book/).
// Rust จะติดตั้ง `cargo` มาด้วย ซึ่งเป็นเครื่องมือที่ใช้สำหรับสร้างและจัดการโปรเจกต์ (หรือ *crates*).
// 
// Next, we have to prepare a workspace for you to conduct your Rust-101 work in, so that you don't
// have to start with an empty file. The easiest way is to
// [download the workspace](https://www.ralfj.de/projects/rust-101/workspace.zip)
// matching the online tutorial. Try `cargo build` in that new folder to check that compiling your
// workspace succeeds.
// (You can also execute it with `cargo run`, but you'll need to do some work before this does
// anything useful.)
//
// Alternatively, you can build the workspace from source by fetching the
// [git repository](https://www.ralfj.de/git/rust-101.git) and running `make workspace`.

// Course Content
// --------------
// 
// Open `your-workspace/src/part00.rs` in your favorite editor, and follow the link below for
// the explanations and exercises. You are ready to start. Have fun!
// 
// ### Introduction
//
// * [Part 00: Algebraic datatypes](part00.html)
// * [Part 01: Expressions, Inherent methods](part01.html)
// * [Part 02: Generic types, Traits](part02.html)
// * [Part 03: Input](part03.html)
// 
// ### Basic Rust
// 
// * [Part 04: Ownership, Borrowing, References](part04.html)
// * [Part 05: Clone](part05.html)
// * [Part 06: Copy, Lifetimes](part06.html)
// * [Part 07: Operator Overloading, Tests, Formating](part07.html)
// * [Part 08: Associated Types, Modules](part08.html)
// * [Part 09: Iterators](part09.html)
// * [Part 10: Closures](part10.html)
// 
// ### Advanced Rust
// 
// * [Part 11: Trait Objects, Box, Lifetime bounds](part11.html)
// * [Part 12: Rc, Interior Mutability, Cell, RefCell](part12.html)
// * [Part 13: Concurrency, Arc, Send](part13.html)
// * [Part 14: Slices, Arrays, External Dependencies](part14.html)
// * [Part 15: Mutex, Interior Mutability (cont.), RwLock, Sync](part15.html)
// * [Part 16: Unsafe Rust, Drop](part16.html)
// 
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unreachable_code)]
mod part00;
mod part01;
mod part02;
mod part03;
mod part04;
mod part05;
mod part06;
mod part07;
mod part08;
mod part09;
mod part10;
mod part11;
mod part12;
mod part13;
mod part14;
mod part15;
mod part16;

// To actually run the code of some part (after filling in the blanks, if necessary), simply edit
// the `main` function.
fn main() {
    part00::main();
}


// Additional material
// -------------------
// 
// There's tons of useful Rust stuff out there, so let me just put links to some
// of the most interesting places here:
// 
// * [The Rust Book](https://doc.rust-lang.org/stable/book/)
// * [The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/)
// * [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
// * The [Rust Subreddit](https://www.reddit.com/r/rust/)
// * A [collection of links](https://github.com/ctjhoa/rust-learning) to blog posts, articles,
//   videos, etc. for learning Rust.
// * For the IRC channel and other forums, see the "Community" section of the
//   [Rust Documentation index](https://doc.rust-lang.org/index.html)
