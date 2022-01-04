// 字符串
/*
Rust 中有两种字符串类型：String 和 &str。

String被存储为由字节组成的vector（Vec<u8>），但保证了它一定是一个有效的UTF-8序列。
String是堆分配的，可增长的，且不是零结尾的（null terminated）。
&str是一个总是指向有效UTF-8 序列的切片（&[u8]），并可用来查看String的内容，就如同 &[T]是Vec<T>的全部或部分引用。


Rust会在栈上存储String对象。这个对象里包含以下三个信息: 一个指针指向一块分配在堆上的缓冲区，这也是数据真正存储的地方，数据的容量和长度。因此，String对象本身长度总是固定的三个字(word)。

*/ 

fn main() {
    // （所有的类型标注都不是必需的）
    // 一个对只读内存中分配的字符串的引用
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，这里并没有分配新的字符串
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 复制字符到一个 vector，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的且可增长的 `String`
    let mut string = String::new();
    for c in chars {
        // 在字符串的尾部插入一个字符
        string.push(c);
        // 在字符串尾部插入一个字符串
        string.push_str(", ");
    }

    // 这个缩短的字符串是原字符串的一个切片，所以没有执行新的分配操作
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    println!("point: {:p}", pangram);
    println!("point: {:p}", &bob);
}
