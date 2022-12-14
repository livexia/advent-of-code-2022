# Advent of Code 2022

链接: [Advent of Code 2022](https://adventofcode.com/2022)

## Day 14 

- 今天的题目并不复杂，细节的处理上也不存在模糊的情况。
- 输入的处理较容易，每一行都是岩石的路径，提取坐标即可。
- 抽象化
    - 构造类型 `Coord` 表示坐标
    - 构造枚举类型 `Material` 表示坐标处的物质，包含 `Rock`, `Air` 和 `Sand` 三种
    - 构造类型 `Cave` 表示洞穴
        - `grid: HashMap<Coord, Material>` 存储坐标对应的物质情况
        - `range_x: (usize, usize)` x 轴的范围
        - `range_y: (usize, usize)` y 轴的范围
        - `sand_src: Coord` 沙子的源头
        - `floor: usize` 第二个部分的地板界限（暂时不考虑）
- 沙子坠落逻辑
    - 每一个单位的沙子都从源头开始 (500, 0)，考虑某一时刻沙子位置为 (x, y)
    - 首先沙子会向下掉 (x, y + 1)，如果目标位置为 Air 那么沙子能够直接掉落，否则
    - 沙子考虑左侧对角线位置 (x - 1, y + 1)， 如果沙子同样不能安放，在考虑
    - 右侧下方对角线位置 (x + 1, y + 1)，如果沙子同样不能安放，那么
    - 目前位置 (x, y) 即是目前这个单元的沙子最终位置。修改 grid 中对应位置的材料为 Sand 。坠落的沙子单元数加 1 ，重新冲沙子源头掉落一个新的单元
- 当沙子掉出石头所能容纳的最大范围的时候，在第一个部分这些沙子会坠入深渊，那么一个单元的沙子会无限的坠落，第一部分就是需要计算当第一个单元的沙子开始坠入深渊时，总共有多少沙子已经坠落而且安放完毕。只需要在沙子的坠落过程中判断当前沙子的位置是否处于深渊（超过输入中所有坐标的边界），如果处于深渊则停止循环，返回计数结果。
- 实际上洞穴下方不一定都是深渊，特别是站在洞穴里网上看，所以第二个部分的洞穴存在地板的限制，地板的高度（y轴坐标）是输入坐标中所有最大 y 轴坐标加二。第二个部分沙子停止坠落的情况，也就是当从源头掉出的沙子已经没有任何地方可以去了，这时沙子停止坠落。那么需要沙子坠落的过程中，记录沙子从源头中掉出后，是否有移动到新的位置，如果没有则停止循环，返回计数结果。
- 逻辑和实现都不难，最后也得到了正确的结果，但是我不满意的地方在于运行的速度较慢，第二个部分的运行时间达到了 1.5s ，可以说是很慢了。利用 [[cargo-]flamegraph](https://github.com/flamegraph-rs/flamegraph)，可以发现运行时间中一个判断位置是否能够安放沙子的函数占用了大量的时间，准确的来说是在对 grid 也就是 HashMap 进行取值的过程耗费了大量的时间。理论上从 HashMap 中取值的时间复杂度是 O(1) ，细看火焰图的时候还可以发现有一半的时间也在计算哈希。为了减少这个部分的时间，可以考虑使用数组替代 HashMap。
- **使用 Vec 替换 HashMap**：如果要使用 Vec 就需要确定洞穴的最大边界，以及沙子可能掉落的边界，可以直接使用第二部分的地板 floor 设定，因为地板是最大的 y轴坐标，也就是说确定了一个最大高度，考虑没有石头的情况，当沙子从 floor 堆叠到沙子的源头时，沙堆的形状是一个三角形，宽度的一半刚好是高度（总是对角线移动），根据这一点以及所有输入坐标中的最大和最小 x坐标，就能确定 Vec 的范围。
    - 首先对输入的所有坐标查找 y 坐标最大值 max_y ，查找 x 坐标的最大值 max_x 和最小值 min_x 
    - 边界值都扩大 2 ，保证第二个部分地板的设定
        - max_y += 2;
        - min_x -= 2;
        - max_x += 2;
    - 计算左边界和沙子左侧最远位置的最小 `min_x.min(500 - max_y)` 作为最终 x 的最小值
    - 计算右边界和沙子右侧最远位置的最大 `max_x.max(500 + max_y)` 作为最终 x 的最大值
    - 所以 `grid` 的大小就是 `max_y + 1 * (max_x - min_x + 1)`
    - **注意后续在进行 `grid` 查找的时候，需要通过 `x - min_x` 来确定最终的索引值。**
    - **这个优化的结果还是比较明显的，两个部分都将运行时间降低到了原有的五分之一。**火焰图显示目前程序运行的时间主要在数组的索引操作上。
    - 进一步的优化，可以发现在沙子垂直掉落的过程中，存在大量重复运算，也就是在 x = 500 的区域上，可以动态更新可能可以放置沙子的 y 值，这样就能免去大量的向下掉落的操作。不确定这个优化能够带来多少的收益，很可能增加了程序的复杂度但效率提升并不明显。
- **可能的数学方法？**也许可以通过计算直接取的空洞区域的大小，这是一个思路，我不够聪明也没有耐心理清这个思路，也没有看见其他人有的实现，所以暂时就这样。


## Day 13

- 初看题目并不复杂，但我在细节的处理上耗费大量时间，没有正确的理解题意导致一直无法得出正确答案。
- 抽象化
    - 每一行输入都是一个数据包，每一个数据包都可以由列表和数字构成，同时列表中又可以由列表和数字构成
    - 构造枚举类型 Packet 
    - 包含 `List(Vec<Packet>)` 和 `Integer(Num)` ，对应列表和数字
    - 其中 `Num` 为 `i32` 的别名
- 输入的处理也不简单，大致上是同检测括号是否正确的思路一致，使用栈完成
    - 对输入按行划分，保留非空行，一行是一个数据包。
    - 将每一行的字符串转为字符数组，直接使用字符数组作为处理栈。
    - 新建结果栈 `stack: Vec<Option<Packet>>`
        - `None` 存在一个列表仍未被构建完成，否则这个列表已经构建完成 
    - 每次从处理栈顶弹出字符（字符串从右到左）
    - 如果字符为 ']' ，表示一个列表结尾，但是因为是从右到左处理，所以是列表的开始，向 stack 中压入 None
    - 如果字符为 '[' ，表示从右到左的过程中，一个列表输入已经完成，那么循环弹出 stacks 栈顶，直到栈顶元素为 None ，那么弹出的 Packet 都是属于外层 Packet 的，构建新的 Packet 后再将结果压入 stack
    - 如果弹出字符为数字，那么表示遇到了 `Integer(Num)` ，因为纯数字的长度不为 1 ，所以需要反复的从字符栈顶端再弹出字符，直到栈顶不为数字。因为是从左到右的，所有得到的数字字符列表是逆序的，需要将列表逆序后再转为数字构造 Packet ，再将结果压入 stack。
    - 如果字符为 ',' 跳过当前字符。
    - 理论上不会遇到其他的字符，可以使用 `unreachable!()` 保证不会又问题
- 对输入进行处理后得到 `Vec<Packet>`
- 第一个部分需要两个两个的划分所有数据包，然后比较两个数据包的顺序，所以需要完成数据包比较的逻辑。
- 比较逻辑，令两个数据包为 left 和 right
    - 如果 left 和 right 都为数字，那么当 left 的数值小于 right 的数值时，两个数据包处于正确的顺序，即 left 小于 right
    - 如果 left 和 right 都为列表，那么依次比较两个列表内的数据包，在依次比较过程中
        - 如果 left 中存在数据包小于 right 中对应位置的数据包，那么 left 小于 right ，**注意在这里只要有一个这样的情况即可，不再需要进行后续比较，我正是错过了这个细节所以浪费了很多的时间**，两个数据包处于正确的顺序。
        - 同样的如果 left 中存在数据包大于 right 中对应位置的数据包，那么 left 大于 right ，两个数据包处于错误的顺序，直接返回比较结果。
        - 如果依次比较过程中 left 和 right 数据包大小都一样（数字一样大），同时 right 的长度大于 left 的长度，那么视作 left 小于 right ，如果长度相同则 left 等于 right ，如果 right 长度小于 left ，那么 left 大于 right。**因为数据包存在嵌套的情况，这时的比较结果会影响外层数据包的比较，所以要严格的传递数据包的比较结果。**
    - 如果 left 和 right 之中存在一个数字和一个列表，那么需要将数字专为包含数字的列表之后再进行比较
- 数据包的大小关系存在三种，大于、小于和等于，但是数据包顺序的正确性只有两种。
- 顺序正确性和数据包大小的对应关系
    - 正确： left < right
    - 错误： left > right
    - 不确定： left == right
- 好在题目的输入中并不存在数据包一致的情况，只需要保证在比较数据包的过程中，确保三种大小关系一致即可。
- 两个部分在严格的实现了比较逻辑后就能够很简单的取得结果。
- 第二部分可以先对输入的所有数据包进行排序，然后在二分查找需要插入的新数据包位置，要插入两个数据包，确定第一个数据包的插入位置时注意要将插入数据包放入数组，然后再查找第二个数据包，确保准确。
- 可以对 `Packet` 实现 `std::cmp::Ord` 来快速使用 `sort` 和 `binary_search` ，当然也可以实现 `Packet::cmp` 而不实现 `trait` ，然后使用 `sort_by` 和 `binary_search_by` 来快速实现第二部分的排序和二分查找。
    - 实现 `Ord` 需要实现 `PartialOrd` 、 `PartialEq` 和 `Eq` ，在这个题目中 `PartialOrd` 是需要实现的，实际上调用 `Ord` 即可，而 `PartialEq` 和 `Eq` 则可以通过 `#[derive( PartialEq, Eq)]` 快速实现
 



## Day 12

- 题意并不复杂，阅读题目的过程中要注意细节
- 输入是由字母构成的网格，网格中由小姐字母代表地势高低，a 最低， z 最高，额外的 S 表示起点，E 表示终点。
- 从 S 出发，要抵达 E ，每次只能在上下左右四个方向中选择一个移动到下一个网格。同时只有当前网格的高度大于下一个网格高度，或者当前网格高度仅比下一个网格高度低一个级别才允许移动。例如 a -> b 是允许的，f -> a 也是允许的，但是 a -> z 就是不允许的。
- 同时 S 的高度等同于 a ，而 E 的高度等同于 z 。
- 要求从 S 到 E 的最短路径，最短路径可以使用 BFS 。上一次写 BFS 的题目已经过去接近两个月了，所以有些生疏，对最短路径的套路也没有很明白，刚开始还尝试用 DFS ，但是对于最短路径而言，广度优先搜索应该是最方便的了。
- 广度优先搜索 BFS
    - 利用 VecDeque 构造队列 queue
    - 初始时队列中仅有初始 S 的位置
    - 每一次从队头取出一个元素，表示当前位置，判断当前位置的上下左右四个位置是否满足移动条件，如果满足则入队
    - **避免死循环**，因为是可以上下左右四个方向移动，所以很可能会导致元素的重复入队。考虑一个位置 A ，如果 A 已经在队列中出现过，因为是深度优先，所以如果后续再将 A 入队，这个时候 A
    所在的路径长度一定长于最早出现 A 时的路径长度（存在环路，无意义的增加了路径长度）。引入 visited 数组，记录位置的访问情况。如果出队的位置已经在访问过，那么就不必要考虑这个位置的后续可能，如果没有访问过，这时需要在 visited 中记录当前位置已经访问过，确保后续不会再次搜索。
- 这是最简单的搜素方式，但是这样只能确定是否有路径。因为是广度优先，所以如果遇到了终点，那么这时的路径就是最短的，同时路径长度就是长度优先搜索的深度。
- 记录深度的 BFS
    - 当队列不为空时
    - 记录队列的长度为 count ，这个长度是当前深度的大小
    - 循环 count 次从队首取出位置，对每一个位置根据上述逻辑继续构造队列。（清空一层）
    - 深度加一
- 当搜索中发现位置即是终点，这时直接返回深度，不需要进一步搜索。
- 刚开始自己的实现其实并没有问题，但是结果却不对，所以我怀疑是自己的 BFS 写错了，刚好中途有事出门，期间一直在想，但是我的实现是没有问题的。回到家后，观察了实际的输入文件，才发现实际输入中 S 的位置并不同示例中是从左上角（0，0）开始的，S 可能是在任意位置的。所以需要自己搜索初始位置。调整了这个细节后就完美解决了第一个部分。
- 第二个部分是第一个部分的简单变体，初始位置并不只有 S ，也可以从高度为 a 的位置开始。最为直接的方法就是找到所有可能的初始位置，对每一个初始位置都进行一次广度优先搜索。在计算所有最短路中的最短路。这个方法要注意很可能有的起点是不可能到达 E 的，要处理到达不了的情况。
- 但是可以逆向考虑第二个部分，终点 E 依旧不变，只不过有多个可能的起点。那么可以考虑从 E 开始，找到终点为 S 或 a 的最短路径。也就是调整 BFS 中的搜索终结条件即可。这个方法不需要进行多次的 BFS 搜索，也能保证一定能够找到可能的最短路径而不需要额外考虑。
- https://github.com/TheAlgorithms/Rust/blob/master/src/graph/dijkstra.rs

## Day 11

- 这道题目的题意有些复杂，不仔细阅读就会出错。输入依旧简单，根据输入构造 Monkey
    - 我最初使用了基本的字符串提取方法，但是在个输入上，使用正则表达式是更加容易的方法。
    - Reddit 讨论区推荐的有 [peg](https://github.com/kevinmehall/rust-peg), [pest](https://pest.rs/), [nom](https://docs.rs/nom/latest/nom/)
- 每一个猴子有两个复杂的动作，检查一个物品时会影响物品对应对担心值（修改），同时又根据新的担心值而决定将物品传递给下一个猴子
- 每个猴子都有不同的修改担心值的方法，由三个部分组成，两个操作数，一个操作符。操作数可以是物品的原有担心值（old）或者是一个常数，操作符可以是乘号或者加号，对应乘法和加法。对两个操作数执行相应操作取得新的结果。
- 传递目标由担心值是否能被一个常数整除而决定，如果能整除则传递给某一个猴子，不能则传递给了一个猴子。每一个猴子都有不同的整除常数，和两个不同的目标猴子。
- 第一个部分中，每一次计算出新的担心值时对担心值除以三，这样可以保证担心值不会越界。
- 但是在第二个部分中，担心值是不能除以三的，而且轮次达到了 10000 轮，所以一定会有越界的情况，需要判断题意自行降低担心值。
- 因为传递是由每个猴子的整除常数确定的，所以要保证降低担心值的过程中不影响整除情况。
- 传递的目标是完全由担心值和整除常数决定的，考虑整除常数 17 ，对于担心值 30 和 13 而言，下一个猴子的位置都是一致的。所以对于某一个猴子而言，可以将担心值降低到担心值对整除常数的余数，而不影响这个物品在这个猴子上的传递目标。
- 能否直接取当前值对当前猴子整除常数的余数作为新的担心值？这是不正确的，考虑当前值为 34 ，整除常数为 17 ，那么余数就是 0 ，假设下一个猴子的操作为 old + 2 整除常数为 18。因为 34 % 17 == 0 而且更新担心值为 0 ，那么等到下一个猴子检查时，担心值被更新为 2 ，被 18 整除不为 0 。但是如果依旧是 34 ，担心值更新为 36 ，能够被 18 整除。可见直接取余数的方法是不正确的，物品传到了错误的猴子手上。
- 可见在降低担心值时不能仅仅考虑当前猴子的情况，也要考虑其他可能有关联的猴子，因为猴子之间的传递链条复杂，大概率每一个物品都会出现在所有猴子手中，所以**在降低担心值时要考虑到所有猴子的情况**。
- 考虑两个整除常数 2 和 3，对于担心值 8 而言，如果要担心值降低后不影响被这两个常数整除的结果，那么应该需要取 8 对 2 和 3 两个数的最小公倍数的余数，也就是 8 % lcm(2, 3) ，即 8 % 6。
- 根据这个思路，在更新物品担心值时，对担心值取所有猴子整除常数的最小公倍数即可。这样就能够保证即使使用了 u64 的情况下也不会出现越界的情况。
- **注意可以不使用最小公倍数（可以直接累积），只不过最小公倍数能更有效的降低担心值，同时在我的输入文件中，所有猴子的整除常数都是素数，所以最后使用的值就是所有整除常数的累积，可能是这个输入文件凑巧导致。**
- 因为第二个部分的循环次数较多达到了 10000 次，所以在不引入这个优化时，即使使用 u128 或者更长的数据类型也有会导致溢出。但是即使在使用这个方法后，依旧需要使用 u64 才能保证不会有溢出。
- 需要注意一个情况，在使用 --release 编译时，即使使用 u32 数据类型也不会产生溢出错误，但是计算结果是错误的，无论是否使用上述方法。也就是说在编译优化的过程中，编译器消除了溢出的问题，但是却导致了计算结果的错误。**这个问题不确定是编译器的 feature 还是 bug ？** 好像是个 feature ？
    - https://stackoverflow.com/questions/71196238/why-does-repeated-multiplication-panic-due-to-overflow-in-debug-mode-when-it-ou
    - https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
    > When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

## Day 10

- 每年都有的指令题来了，输入的处理依旧简单
- 这个 CPU 很简单，只有一个寄存器和两条指令
- 指令 noop 什么也不做，需要一个周期
- 指令 addx a 寄存器自增 a ，需要两个周期
- 寄存器的值只有在指令完全执行完成后才会更新
- 构造 CPU ，包含寄存器 register ，程序 program ，程序计数器 pc
- 仅有一个寄存器，所以 register 类型为 i32
- program 为指令列表
- pc 为指令指针，当前执行指令位置
- 当前指令为 noop 时，pc 加一即可
- 当前指令为 addx 时，因为这个指令需要两个周期，所以在第一次 addx 时，不对 register 进行修改，下一个周期再对 register 修改，然后 pc 加一
- 所以 CPU 中需要另外的计数器 cycle ，用于记录当前指令的执行周期数。因为只存在一个指令需要两周期，所以可以使用 bool 类型进行计数器。初始为 false ，表示不需要额外周期执行当前指令，只有当 pc 指向 addx 时，cycle 置为 true ，表示需要额外的一个周期。在下一个周期中，pc 虽然依旧指向 addx ，但是因为 cycle 为 true ，表示指令需要的额外周期已经完成，这时可以对 register 修改，同时 pc 指向下一条指令。
- 在指令寻址的部分，程序可能是循环执行的，即当程序运行结束时，pc 需要从头开始。在这个题目中其实不涉及这个问题，因为程序运行周期为 240 ，刚刚好在两个部分问题的范围内。按照往年题目，后续肯定有类似的扩展题，后续可能要考虑这一点。
- 第一部分需要计算 CPU 在某一个执行周期中寄存器的值。根据上述的 CPU 实现，只要按照周期运行即可，在特定周期时记录寄存器值计算信号强度。因为要求计算的是执行周期中寄存器的值，这时指令还没完成执行，寄存器的值没有更新，是和周期前寄存器值一致的。而且当前的实现，一个周期是不存在执行中的情况的，所以直接用当前周期执行前寄存器的值。
- 第二部分看似复杂，需要绘制像素。绘制一个像素的过程分为两个部分：
    - 首先是 `sprite` ，可以视作画笔，宽度为 3 
        - `sprite` 的中间位置是寄存器的值，这个值是可能小于 1 的，所以画笔有作用的实际宽度会小于 3 
    - 然后是屏幕的部分，每一个计算周期中，只会更新一个像素，可以视作画布
    - 只有当画笔落在了画布上，像素才会被点亮
    - 令屏幕为 `screen: [[false; 40]; 6]` 初始时所有像素都没有点亮为 `false` ，屏幕宽度为 40 高度为 6
    - 屏幕的绘制方式是从左到右，从上到下，最初绘制的像素是 (0, 0) ，最后绘制的像素是 (5, 39)
    - 令当前绘制行 `cur_row` ，当前行中的位置 `cur_pos` 那么当前绘制的像素就是 `screen[cur_row][cur_pow]`
    - 那么只有当绘制像素落在 `sprite` 的范围内，像素才会被点亮，否则不变
        - 可以计算 `cur_pos` 和 `sprite` 中间位置的距离，如果距离小于 2 ，那么绘制像素落在 `sprite` 中
    - 当绘制的位置超出了行的边界，表示需要绘制下一行
- 因为绘制也是在一个执行周期中完成的，所以绘制是发生在周期完成前，需要先绘制，在执行周期
- 执行完所有的周期，第二个部分也就是 240 个周期，执行再多的周期也不会对屏幕有影响。这时得到的 screen 就是像素的明暗情况，根据对应位置值，绘制 '#' 和 '.' ，打印结果即可取得答案。我还记得写 18 年的题目的时候，也是一个绘制题，我画出了正确的结果，但是却认错字，今年的结果还是很清晰的。


## Day 9

- 第九天的问题难度并不大，输入的处理上也不复杂，在细节上需要考虑仔细
- 输入处理
    - 构造枚举类型 Move ，类型内部数据为步数
    - 实现 trait FromStr 将一行输入构造对应的 Move 类型
- 第一个部分的绳子很短，只有两个结，尾部的结 T 随着头部结 H 移动 
- T 跟随 H 移动的规则如下
    - 如果 H 和 T 直接相邻或者重合那么 T 不移动
    - 如果 H 和 T 处于同一行或同一列，而且距离为 2 ，那么 T 向 H 的方向移动一步。如果每一次移动都严格按照移动规则进行移动，当 H 和 T 同属一行或一列时，他们的距离不可能超过 2 ，同时 T 当新位置就是 H 和 T 的中位，所以可以不用考虑 H 的移动方向，直接计算中位即可 T = (H + T)
    - 如果 H 和 T 处于对角线，同时距离为 2，那么这个时候 T 不移动
    - 如果 H 和 T 的距离大于 2 ，那么 T 的移动方向是对角线，这个时候 T 有四个可能的移动方向，可以发现四种情况中只有一种可以缩短 H 和 T 的距离，所以遍历四种可能，确定唯一的移动路径即可
    - 更加简便的规则：
    > https://github.com/ropewalker/advent_of_code_2022/blob/master/src/day09.rs
- 移动的情况是随意的，所以假想的地图应该是无限大的，那么利用 Vec 来抽象地图就不方便了，而且题目的要求中并不需要真正的抽象地图，而只需要计算某一个绳结所有的访问位置，那么可以利用 HashSet 来进行存储，值为对应的绳结坐标即可。
- 同样的对于 H 和 T 也只需要关注每一颗的坐标即可。
- 对 Move 实现方法 get_step 和 move_fn 来快速取得 H 需要移动的步数和移动的方法，move_fn 方法返回函数闭包，能够传入参数快速计算出 H 的下一个位置。
- 第一个部分的解答就很容易了，因为只有两个结 H 和 T ，为了防止混乱（步子迈太大容易扯到自己），所以一步一步计算，对于每一步，首先利用 move_fn 的方法移动 H ，再根据移动规则移动 T ，将 T 的新位置存入 HashSet 。最后 HashSet 的长度即是 T 到访过的所有位置。
- 第二个部分看似复杂，绳结的数量增加到了 10 个，构建 ropes ，c长度为 10 ，初始时每一个绳结的位置都为 (0, 0)，索引 0 为 H 的坐标，索引 9 为 T 的坐标。依旧是一步一步的考虑，对于每一步，移动总是从头部开始，所以当 i 为 0 时，利用 move_fn 移动头部。对于其他绳结，利用移动规则计算下一个位置 `ropes[i + 1] = move_tail(ropes[i], ropes[i + 1])` 可以理解 i 是头部和 i + 1 是尾部，这是第一部分。需要记录尾部的访问情况，所以当 i 为 8 时，也就是计算 `ropes[i + 1]` ，这个时候将 `ropes[i + 1]` 的位置存入 HashSet 。同样的最后 HashSet 的长度即是 T 到访过的所有位置。
- 如果花费大量的时间，也是可以不一步一步的来，但是那就要花费大量时间分析 H 和 T 的对应情况，而且这个问题需要确定 T 的移动路径，那么对这个问题来说可能没有必要。

## Day 8

- 今天这一题考虑过度，导致花了很长的时间，最后是直接暴力完成的，运行速度不慢
- 输入的处理很简单，直接按照字符划分即可，再将每个字符转为数字即可
- 对于每一个位置，只存在四个观测路径，朝上下左右四个方向看，直到实现被阻挡或者尽头（边界）
- **搜索最远可见树的位置**：考虑一个位置 (x, y)，树高为 h， 从这个位置向左遍历所有树高度小于 h 的位置，当遍历到的位置树高大于 h 或 遍历到边界时，遍历结束，结束时的位置即为 (hx, hy)
- 对于第一部分，如果位置 (hx, hy) 是处于边界的，那么起始位置 (x, y) 是可以从边界之外看见的树。那么只要遍历网格中所有的位置，同时对每一个位置，进行四个方向的搜索，最后就能确定所有能够从边界处看见的树。
- 对于第二个部分，从位置 (x, y) 搜索到最远可见的树的位置 (hx, hy) ，在这两个位置间存在树的数量即是计算分数的一部分，因为搜索只能是笔直方向的，所以两个位置的差距即是可见树的数量，如果 (hx, hy) 是在边界上，那么在这之外是看不见一棵树的，但是如果 (hx, hy) 不在边界上，那么视线被 (hx, hy) 的树阻挡，这一棵树也要计入，需要在结果上加一。计算四个方向上可见树的数量，再累积即可。最后对所有位置上的分数取最大值即可（或者搜索时同时计算最大值）。
- 考虑网格的大小为 n * n （方便起见设为正方形），每一次搜索的路径都是笔直的，所以一个位置对一个方向上的搜索，最多需要比较 n 次，时间复杂度为 O(n) ，那么所有位置对一个方向的搜索的时间复杂度是 O(n^3) ，四个方向上搜索的时间复杂度就会是 O(4n^3) 。
- **可能有的优化**：正是这个地方让我花费了大量的时间，最初我并没有意识到搜索的路径是笔直的，所以是在四个方向上进行四次毫无关联的搜索。因为给定输入的网格大小只是 100 ，所以即是没有优化，最后的时间复杂度也不会过高，程序依旧能较快完成。
- **优化——记忆化搜索**：
    - 第一部分：考虑输入中的一行 123 ，从左到右依次编号 1、2、3，那么考虑向左搜索，搜索 1 号时，不需要进行比较，已经处在边界，所以 1 肯定是从左边界可见的，搜索 2 号时，需要和 1 号进行比较，抵达边界不再需要其他的比较，所以 2 也是左边界可见的。考虑 3 号的搜索，默认情况下 3 号要和 2 号比较，然后再同 1 号比较，所以需要比较两次，但是在和 2 号进行比较的时候，因为 2 号已知可以被看见，所以只需要进行一次比较即可。这个优化的麻烦点在于四个方向上的搜索是独立的，所以需要进行四次记忆化。
    - 第二部分也可以用这样的方法进行记忆化，当前位置为 (x, y) 在比较下一个位置 (x1, y1) 时，如果当前树高于下一个树，那么可以直接和下一个树可见位置 (hx1, hy1) 进行比较，因为 (x1, y1) 位置的树高一定大于 (hx1, hy1) 处的树高，那么中间那些重复的比较就可以略过，减少了计算。同样的这个记忆化在不同方向上是独立的，需要单独存储。
- 现在看来好像这些优化也并非特别复杂，这是因为我已经完成了基础版本的代码，在这基础上再进行优化会比从写第一行代码时就开始考虑优化要容易很多。可以说是步子迈的太大以至于浪费了时间，也弄乱了思路。

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
    - 那么只要将代表集装箱的大写字符的位置除以 4 就可以得到栈的索引，而且因为索引从 0 开始，所以得到的就是具体的索引，不必加一
    - 当需要将集装箱压入的栈不存在时，即索引大于栈的数量时，需要构造空栈。
    - 而且根据输入是从顶部开始入栈，最后得到的结果是相反的，所以构造完之后也需要对每一个栈进行逆序操作。
- 第一个部分只要按照栈的要求即可，根据输入的步骤不断的进行 pop 和 push 操作即可，注意输入中栈的索引是从 1 开始
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
    - 如果不进行排序，需要进行两次比较，在这个部分两次比较的方法更加不容易出错。 (p1.0 <= p2.0 && p1.1 >= p2.1) || (p1.0 >= p2.0 && p1.1 <= p2.1)
- 第二个部分需要判断每行的两个区间存在重叠的情况
    - 可以根据区间的起点进行排序，设起点较小的区间为 p1 ，这时如果 p2 的起点不大于 p1 的终点，那么必然存在重叠
    - 因为按照起点排序，所以 p2 的起点一定大于等于 p1 的起点，如果两个区间重叠，那么只要 p2 的起点都在 p1 区间中，这时两个区间一定存在重叠。
    - 这里的排序并没有第一个部分排序时的忧虑，考虑两个区间起点相同（这时已经存在重叠），这个起点都会小于两个区间的终点，也就是说一定存在重叠。

## Day 3

- 第一个部分和第二个部分的问题是一致的：取得所有都出现在几个字符串中的字符
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