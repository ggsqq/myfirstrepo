use itertools::EitherOrBoth::{Both, Right};
use itertools::Itertools;

fn main() {
    // 1. interleave 就是轮流消耗 2 个 iterator,直到他们都消耗完.
    let v = (1..7).interleave([-1, -2]).collect_vec();
    assert_eq!(v, vec![1, -1, 2, -2, 3, 4, 5, 6]);

    // 2. 轮流消耗 2 个 it,直到一直消耗完
    let v = (1..7).interleave_shortest([-1, -2]).collect_vec();
    assert_eq!(v, [1, -1, 2, -2, 3]);

    //  3. 往 it 中轮流插入一个元素 ⚠️ 未来这个同名方法可能要加到标准库里,所以编译报警
    #[allow(unstable_name_collisions)]
    let v = (0..3).intersperse(8).collect_vec();
    assert_eq!(v, [0, 8, 1, 8, 2]);

    // 4. 同intersperse, 不过可以使用闭包 ⚠️ 未来这个同名方法可能要加到标准库里,所以编译报警
    #[allow(unstable_name_collisions)]
    let v = (0..3).intersperse_with(|| 8).collect_vec();
    assert_eq!(v, [0, 8, 1, 8, 2]);

    // 5. 会按照最长来组合一起, 返回一对一对的数据. 然后用 enum 来告诉你是 Both, Right, Left
    let list = (0..1).zip_longest(4..8).collect_vec();
    for v in &list {
        println!("{:?}", v);
    }
    assert_eq!(list, vec![Both(0, 4), Right(5), Right(6), Right(7)]);

    // 6. 闭包的 fn 有 it 的引用,可以随意生成任意的返回
    let v = (0..5)
        .batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => None,
                Some(y) => Some((x, y)),
            },
        })
        .collect_vec();
    assert_eq!(v, vec![(0, 1), (2, 3)]);
}
