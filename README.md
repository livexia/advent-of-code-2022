# Advent of Code 2022

链接：[https://adventofcode.com/2022](https://adventofcode.com/2022

## Day 8

- 

## Day 7

- 这一题要求根据命令和结果构造存储结构，应该是不难的一题，但是我却花了两个小时
- 输入包含两个部分，命令和命令输出结果
- 只有两条命令 cd 和 ls 
- cd 表示进入目录，这个命令无输出结果
- ls 表示输出当前目录结构
- ls 的命令输出结果中也包含两部分
- 例如 dir a 表示存在子目录 a
- 和 14848514 b.txt 表示存在文件 b.txt 大小为 14848514
- 对输入的处理并不难，但是 cd 命令中存在两个特殊的情况， `cd /` 和 `cd ..` ，也就是说进入子目录后需要能够回到上层目录和根目录
- 第一部分需要在确定目录结构之后，计算每一个目录的大小，再计算目录小于 1000000 的目录大小总和
- 第二部分需要确定再删除了哪一个目录后能够空出需要的空间，所以同样需要计算每一个目录的大小。
- 如果能够正确的表示目录结构那么就能够轻易的求解。
- **构造树有些大材小用，如果要计算目录大小，可以模拟完成。**处理输入时记录文件夹结构，同时计算文件夹下文件大小的总和，完成输入处理后，再递归计算子文件夹大小，这样就能确定所有文件夹的大小。
    - 转移方程 f(dir) = sum{f(sub_dir)} + sum{f(file)}
    - 边界情况：当文件夹不存在子文件夹时，f(dir) = sum{f(file)}

### 构造树

1. 初步构造 Dir 
```rust
struct Dir {
    name: String,
    sub_dir: HashMap<String, Dir>,
    files: HashMap<String, File>,
    size: Option<usize>,
    parent: Option<Dir>,
}
```
这时最初的文件夹思路，但是这样的做法存在严重的问题，问题就在于对上层文件夹的访问，如果将文件夹结构视为树，那么也就是子节点需要访问父节点，那么这样的 Dir 是无法满足需求的，虽然也有 parent 但是会导致所有权的问题，一般来说构造树的时候需要使用 `Rc<RefCell<Box<T>>>` 但是我并不想这样做，太复杂了。所以我参考了 https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/ 中构造树的方法。

2. 增加 Dirs ，利用 Vec 和节点索引值来构造树结构
```rust
struct Dirs {
    dirs: Vec<Dir>,
    next_index: usize,
}
```

3. 调整 Dir 为
```rust
struct Dir {
    id: usize,
    sub_dir: Vec<usize>,
    table: HashMap<String, usize>,
    files: HashMap<String, usize>,
    parent: usize,
}
```
- sub_dir 中只保存子目录的索引值
- table 中保存子目录名称和索引值的对应
- id 当前目录的索引值

这样修改之后既可以实现子节点快速访问父节点，又可以保证树的结构


4. 构造 File 

File 较为简单，只需要包含文件名和大小，可以视为树的叶子节点。可以优化直接优化为 HashMap ，而不需要额外的结构，直接用哈希表存储文件名和大小即可。
```rust
struct File {
    name: String,
    size: usize
}
```

### 模拟

1. 首先对输入进行处理
    - 构造栈 pwd 存储档期当前目录的绝对路径
    - `cd ..` pwd 弹出栈顶
    - `cd \` pwd 设为 `vec!["/"]`
    - `cd name` pwd 压入 name
    - 对于 ls 命令，需要完成两件事情，
        1. 记录当前路径下所有的子目录
        2. 记录当前路径下所有文件的大小和
    - 构造 `sub_dirs: HashMap<String, Vec<&str>>` 来存放每一个路径对应的子目录
    - 构造 `sizes: HashMap<String, usize>` 存放每一个路径的大小
    - 对输入进行处理时，是没法直接计算出文件夹大小的，读取当前文件夹时子文件夹的情况还没读取和处理，所以输入处理时只能计算出一部分的文件夹大小（文件部分）
2. 当已经完成所有文件结构的确定，即完成输入同时也完成构造 `sub_dirs` 时 
    - 从根目录计算每一个文件夹的大小
    - 如果当前目录没有任何子文件夹，那么输入时计算的所有子文件大小总和即是当前文件夹的大小，不更新 `sizes`
    - 如果当前目录存在子文件夹，那么递归计算子文件夹的大小，确定所有子文件夹大小后，更新 `sizes`


## Day 6

- 依旧是不复杂的一题，对输入的处理也很简便
- 读入字符串，转为字符数组
- 第一个部分要求寻找最早出现的四个完全不同的连续字符的位置
- 第二个部分要求寻找最早出现的十四个完全不同的连续字符的位置
- 考虑直接暴力的方法，单指针，从零开始，每次截取所需要的长度的字符数字，存入 Set ，判断 Set 的长度是否不变，如果不变说明截取部分不存在重复，如果发生变化则向右移动指针，继续判断。

## Day 5

- 这一题的难点在于如何处理输入，如果不从文件中读取，而是直接根据输入手动写栈，那么最后的结果应该还能向前
- 首先确定每一行的输入类型
- 如果是以 move 开头，那么输入为操作类型，根据空格进行划分，取出中的数字，第一个数字表示移动数量，第二个数字为初始栈，第三个数字为目的栈。
- 如果为空行则跳过
- 其他的行都是初始栈的一部分，输入是很形象的栈，所以第一行是所有栈的顶部，自上而下栈顶到栈底。
    - 每一个大写字母代表一个集装箱，其他的字符都无意义，但是可以发现即使是空的栈，输入中也包含了空格
    - 利用 char_indices 获取每一行中的字符，以及对应位置，需要将字符位置对应到具体的栈索引
    - 正常情况下，输入中表示集装箱需要使用三个字符，同时使用一个空白和隔壁的栈表示分割，可以考虑从字符 '[' 到空白的四个字符都属于当前栈，第一个栈的字符位置应当是 0 到 3（都包涵），第 N 个栈的字符位置应当是 (N - 1) * 4 到 (N - 1) * 4 + 3
    - 那么实际上只要将代表集装箱的大写字符的位置除以 4 就可以得到栈的索引，而且因为索引从 0 开始，所以得到的就是具体的索引，不必加一
    - 当需要将集装箱压入的栈不存在时，即索引大于栈的数量时，需要构造空栈。
    - 而且根据输入是从顶部开始入栈，最后得到的结果是相反的，所以构造完之后也需要对每一个栈进行逆序操作。
- 第一个部分实际上只要按照栈的要求即可，根据输入的步骤不断的进行 pop 和 push 操作即可，注意输入中栈的索引是从 1 开始
- 第二个部分不再是一个一个移动，而是一次性移动好几个，不是纯粹的栈操作。当然可以全都 pop 存放到中间栈，再将最后的结果存入目的栈。但是在这里我直接使用了 Rust 中 Vec 的方法，利用 split_off 直接截取需要移动的元素，然后再利用 extend_from_slice 移入目标栈。

## Day 4

- 这是一道区间题，判断两个区间的重叠情况
- 对输入进行预处理
    - 首先对每行根据 ‘,’ 进行分割
    - 再对每个部分根据 ‘-’ 进行分割
    - 再将字符串转为数字构造具体区间
- 第一个部分需要判断每行的两个区间存在完全重叠的情况
    - ~~可以根据区间的起点进行排序，设起点较小的区间为 p1 ，这时如果 p2 的终点小于 p1 的终点，那么完全重叠~~
    - 仅仅根据起点进行排序是不够的，例如对于区间 (14, 39) 和 (14, 85) ，按照起点排序，那么顺序不变，这时 p2 区间的终点是不小于 p1 区间的终点的，所以按照这个逻辑这两个区间是不存在完全重叠的，和实际相反。
    - **排序逻辑：**按照起点排序，起点相同时按照终点倒序，即终点较大的区间为 p1 。
    - 如果不进行排序，需要进行两次比较，实际上在这个部分，两次比较的方法更加不容易出错。 (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1)
- 第二个部分需要判断每行的两个区间存在重叠的情况
    - 可以根据区间的起点进行排序，设起点较小的区间为 p1 ，这时如果 p2 的起点不大于 p1 的终点，那么必然存在重叠
    - 因为按照起点排序，所以 p2 的起点一定大于等于 p1 的起点，如果两个区间重叠，那么只要 p2 的起点都在 p1 区间中，这时两个区间一定存在重叠。
    - 这里的排序实际上并没有第一个部分排序时的忧虑，考虑两个区间起点相同（实际上这时已经存在重叠），这个起点都会小于两个区间的终点，也就是说一定存在重叠。

## Day 3

- 第一个部分和第二个部分的问题实际上是一致的：取得所有都出现在几个字符串中的字符
- 可以进一步化简为，取得所有都出现在几个序列中的元素
- 第一个部分每一行输入需要一分为二，然后进行计算
- 第二个部分则是每三行输入需要进行计算
- 通过 b as u8 - 'A' as u8 + 27 可以将大写字符转为题目要求的权重
- 通过 b as u8 - 'a' as u8 + 1 可以将小写字符转为题目要求的权重
- 每个字符可能在一个序列中重复出现，所有对于一个序列中的字符，只计算一次出现的情况
- 第一部分：
    - 构造一个 dup 数组，元素类型为 (bool, bool) ，长度 53 ，数组下标对应相应字符权重对应字符
    - 第一个元素表示当前字符在前一半的输入中出现情况，第二个元素表示当前字符在后一般的输入出现情况。
    - 当元素值为 (true, true) 时则表示当前字符即出现在前一半输入，又出现在后一半输入。
- 第二部分和第一部分一致，只不过 dup 数组的元素需要增加一个元素，应对新增的第三个输入
- 更多解法
    - 使用位运算，最多有 52 个不同字符，可以利用 u64 来保存一个输入序列的出现情况，最后计算结果时可以对不同序列的出现情况进行与运算，然后再计算位为 1 的位置和
    

## Day 2

- 依旧考察的是对于输入的处理，并不涉及复杂逻辑。自己实现的时候在对于输入的解析上过于复杂了，如果仅仅为了得出结果，完全可以直接处理字符，但是这样的实现便于阅读，也可以练习实现一些常见的 Trait
- 第一个部分输入均为手势
    - 构造枚举类型 Shape 对应手势
    - 通过实现 Trait FromStr 可以很容易的将 A B C X Y Z 六个不同的字符串对应到不同的手势
    - 通过实现 Trait Sub 实现不同手势间的计算结果，实现结果计算
        - 也可以实现 Traid Ord 实现不同手势的大小比较，同样实现结果计算
- 第二个部分第一列输入为手势，第二列输入为结果
    - 对第一列依旧实现字符解析，对应 Shape
    - 构造枚举类型 Outcome 对应不同的结果
    - 不再实现 Trait ，而是直接利用 match ，根据两列的输入计算结果
- 分数计算：根据给出的手势和结果进行计算每一轮的分数
    - 石头：1 分
    - 步：2 分
    - 剪刀： 3 分
    - 输：0 分
    - 平：3 分
    - 赢：6 分

## Day 1

- 首先对输入进行处理
    - 输入数据为每个精灵所携带（不同）食物的卡路里数
    - 一个精灵可能对应多个卡路里数
    - 不同精灵通过空行分割
    - 最初设想保留每一个卡路里数
- 第一个部分计算最大的精灵携带食物卡路里数
    - 不需要每一个精灵中每一个卡路里数
- 第二个部分计算前三的精灵携带食物卡路里数
    - 同样不需要每一个分别的卡路里数
- 可以在输入时就计算每一个精灵携带食物的卡路里总数
- 第一个部分直接使用 max 取得，时间复杂度应该是 O(n)
- 第二个部分将转化后的输入数据存入最大堆 (O(n))，取三次堆顶，累加即是结果，时间复杂度是 O(logn)
- 也可以直接对输入数据进行反向排序 O(nlogn)，第一个部分取第一个元素，第二个部分的问题则是前三元素的和