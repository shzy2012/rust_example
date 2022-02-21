/*
冰雹的最大魅力在于不可预知性

冰雹猜想：一个正整数x，如果是奇数就乘以3再加1，如果是偶数就析出偶数因数2ⁿ，这样经过若干个次数，最终回到1。
无论这个过程中的数值如何庞大，就像瀑布一样迅速坠落。而其他的数字即使不是如此，在经过若干次的变换之后也必然会到纯偶数：4-2-1的循环。

*/
pub fn hailstone(n: u32) -> u32 {
    let mut next = n;
    let mut count = 0u32;
    while next > 1 {
        next = if next % 2 == 0 {
            next / 2
        } else {
            next * 3 + 1
        };

        count += 1;
    }

    count
}

#[test]
fn hailstone_0() {
    assert_eq!(0, hailstone(0));
}

#[test]
fn hailstone_1() {
    assert_eq!(0, hailstone(1));
}

#[test]
fn hailstone_10() {
    assert_eq!(6, hailstone(10));
}

#[test]
fn hailstone_27() {
    assert_eq!(111, hailstone(27));
}
