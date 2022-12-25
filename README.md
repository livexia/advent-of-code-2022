# Advent of Code 2022

链接: [Advent of Code 2022](https://adventofcode.com/2022)

## 进一步学习

- 对所有代码进行 clippy 检查并修改
- Day 22
- Day 19 (进一步性能提升)
- Day 18
- Day 17
- Day 16 (测试不通过)

## 总结

今年自己可以明显感觉到对于路径查找、动态规划的题目更加熟练，虽然期间也有几天的题目花了非常多的时间，但是基本上自己都能够独立完成，看了去年的排名和时间，可以发现好像去年的结果好像稍好看一些，但是去年如果自己几个小时想不出来，我就直接看社区答案了。今年有三天第二部分应该是看了社区的思路，但是今年借鉴的成分远比去年少，更多的还是自己实现。相比于去年，今年对于记忆化搜索认识更加彻底，意识到在何种场景下增加缓存能够加速程序。而对于 DFS 、 BFS 和 DP 去年可能都仅有简单理解，而今年自己则更加的熟练，对于去年不熟练的迭代和递归，今年也可以熟练运用和理解。虽然今年也有不少需要构建树的题目，但是我通过侧面的方法规避了直接构造树结构，所以对于树的构造依旧不熟练，我现在对于 Rc RefCell 和其他一些的智能指针已经有了足够多的认知，但是相关的练习依旧较少，这是后续需要提升的。在性能分析和程序风格上，我使用 cargo flamegraph 对程序性能进行分析，通过分析可以确定程序优化点，确定是否有必要进一步对当前的方法进行优化。使用 cargo clippy 对 rust 代码进行规范和风格检查，虽然在最后几天才开始使用这个工具，但是这个工具的效益已经让我觉得必不可少，所以在最后我也会对今年 AOC 的之前代码都进行 clippy 检查。今年终于在最后一天的 part 2 挤进前 1000 ，明年加油。

## Day 25

- 依旧是最后一天圣诞节的简单题。
- 需要实现数字的进制转换，类似于五进制和十进制间的转换。
- 题目给出的并非标准的五进制，并不是从 0 到 4 五个数字表示，而是由 = - 0 1 2 五个字符表示，其中 = 表示 -2 ，而 - 表示 -1。
- 首先是五进制转十进制，这很容易实现，输入为字符串，从字符串尾部（数字低位）开始读取，根据读入的字符对当前位的基进行乘法运算，合并每一位的计算结果即可得到十进制表示，基从 1 开始每一位之后都需要乘 5 。
- 十进制转题目中的五进制是比较复杂的，对于一般的五进制，一般是对十进制取 5 的余数，然后将十进制整除 5 ，直到十进制为 0 ，只这样可以得到从最低位到最高位的五进制表示。但是题目中的表示形式是 = - 0 1 2 ，可以发现实际上 = 就是 5 - 3，- 则是 5 - 4，也就是说当余数为 3 的时候，如果我们将其视为 5 ，那么就需要减 2 ，也就是 = 符号，同理 - 也是如此。所以当余数为 3 时，向结果中推入 = `snafu.push('=')`，但是因为借了 2 给十进制，所以需要对十进制整除 5 的结果加一 `num += 1`。
- 实现这两个方法就可以解决第一部分的问题，因为是最后一天，所以第二部分是直接可以通过的。

## Day 24

- 今天的题目并不困难，但是存在一个边界情况没有考虑到，浪费了不少时间。
- 输入为当前时刻的地图以及暴风雪情况。
- 地图的每一个坐标处存在三种情况：墙、空地、暴风雪，人和暴风雪无法移动到为墙的坐标，人无法移动到为暴风雪的坐标，暴风雪可以移动到为暴风雪的坐标。
- 存在两个移动的物体：
- 第一个是人也就是我，需要从起点移动到终点。在一分钟内人可以上下左右移动，**如果情况允许**也可以不移动。移动规则如下：
    - 不能移动到地图边界之外
    - 不能移动到墙
    - 不能移动到为暴风雪的坐标
    - 如果下一分钟暴风雪会移动到当前位置，那么不能在原地不动
- 第二个则是地图上到处都有的暴风雪，暴风雪的移动规则如下：
    - 每个暴风雪占据一个坐标，由输入的字符表示移动方向
    - 移动过程中暴风雪的方向不变
    - 每次移动花费一分钟，一次移动最多移动一个方格
    - 当移动到边界时，方向不变的从另一侧边界继续移动，也只需要一分钟
    - 如果多个暴风雪的下一个位置相同，那么下一分钟时他们同时存在与那个位置，不影响暴风雪的移动
- 可以发现人的移动情况是受制于暴风雪的位置，而暴风雪的移动则与时间和地图大小有关。
- 构造 struct Map 表示某一时刻的地图情况，因为输入地图大小较小，同时无论是人还是暴风雪的移动都被限制在一定范围内，并不是无限平面，所以直接使用二维 Vec 存放当前地图情况。每一个元素表示当前位置的类型，因为可能存在多个暴风雪，每一个元素也是一个 Vec 。
- 暴风雪直接用输入的字符表示，在每一分钟的移动中，首先构造同样大小的 Vec 存放下一分钟的地图情况。遍历每一个坐标，如果当前坐标处为一个或多个暴风雪，移动这些暴风雪（考虑边界情况），在新的 Vec 中重新存入暴风雪。所有坐标完成便利后，替换 Map 中的 Vec ，暴风雪移动完成。
- 在实现了暴风雪的移动之后，我尝试进行无限次的移动，同时将每一分钟的地图情况存入集合，我想要判断是否会出现循环的情况，果不其然发现了循环，对于示例输入循环是 12 ，我的输入循环则是 600 ，也就是说可以将时间与地图对应，而不需要一边移动一边更新地图的情况。实际上循环的大小就是可移动区域长宽的最小公倍数，示例输入是 4 和 6 最小公倍数是 12 ，而我的输入则是 25 和 120 最小公倍数刚好是 600 ，向上或者向下的暴风雪，最少移动距离为可移动区域的高度，就可以回到原处，同样的向左向右则是可移动区域的宽度，而要所有的暴风雪都再次回到原地，那么最少就需要长宽的最小公倍数。
- 确定了暴风雪的移动情况存在循环，那么就可以考虑人的移动。
- 因为要计算最短路径，无权图直接使用**广度优先搜索**，初始位置为起点。
- 在任意坐标处，人的移动情况最多有五种，上下左右和不动，这些移动情况收到当前时间的地图情况影响，虽然题目中描述人和暴风雪的移动是同时的，但不妨想象暴风雪移动先进行，然后人再进行移动。
- 队列的元素为当前人的坐标和下一分钟的时间，因为时间和地图情况存在对应和循环关系，实际上元素可以理解为是人的坐标和下一分钟暴风雪的情况，或者是上一个坐标和当前暴风雪情况。
- 在队列不为空的情况，每次从队首弹出坐标和当前地图情况，计算当前坐标所有可能的下一个位置，判断这些坐标在当前地图中是否允许，如果允许则直接将该坐标和加一点时间压入队列尾部，如果这些坐标中包含了目的地，则搜索完成，直接返回当前时间。
- 在搜索过程中增加访问表，如果当前人的位置和下一刻暴风雪的情况已经出现过，那么这个路径无需再进行搜索，之前的路径一定短与当前路径。因为暴风雪的移动情况存在循环，与时间存在直接的对应关系，所以无需用坐标和暴风雪的情况作为访问表的元素，而可以直接使用转换后的时间（最小周期的时间即可）。同时对与搜索的元素也要在访问表中插入坐标和对应转换后的时间。
- 第一部分就是进行一次搜索，第二部分则是进行三次搜索。
- 在实现的过程中有一个人移动情况的细节被我忽略了，对于在原地等待的情况，这个时候不仅要考虑下一分时，我是否还能在原地等待，所以也要判断下一时刻当前位置是否有暴风雪。然后也要考虑起始位置的情况，很可能在一开始我就无路可去，也就是说我只能在原地等，在这里我做了一个假设，也就是起点和终点都是一定不会有暴风雪的，实际上在暴风雪的移动中我也并不考虑这两个位置，所以在这个情况下如果我在起点，那么我想要等多久都可以（拖延症晚期患者）。再处理这个两个细节问题之后，轻松解决。
- 优化 1 ，使用更加高效的方式代表暴风雪的情况：
    - 因为每一行输入的长度不大于 128 所以可以使用 u128 代表一行的暴风雪出现的情况
    - 同时在初始状态，每一个方格仅有一个暴风雪（如果一个坐标处存在两个以上的暴风雪，无法决定任一的方向，题目无解）。
    - 所以理论上再移动的过程中，一个坐标最多存在四个方向不同的暴风雪。
    - 根据这一点可以使用4个 u128 表示一行中每个位置每一个方向暴风雪的出现情况。
    - 这个方法可以降低预构造哈希表的难度。存储空间由 u8 * 4 * 122 * 27 到 4 * 27 * u128
    - 参考：https://www.reddit.com/r/adventofcode/comments/zu28ij/comment/j1gww24
- 优化 2 ，可以确定暴风雪的出现情况是与时间直接关联的，同时周期是同可移动范围的长宽相关，那么在预构造的过程中完全可以直接计算所有周期，而不必使用 HashMap 尝试的方法。需要实现 lcm 函数计算最小公倍数。
- 这两个优化提升并不明显，特别是第一个优化，这个优化方法会增加理解难度，虽然我实现了，但是在这个场景里不是特别有价值。

## Day 23

- 依旧是需要注意细节的模拟题。
- 输入为地图，# 为精灵的初始位置。
- 题意分析
    - 精灵需要移动到特定位置，以类似于回合制的方式进行移动。
    - 每一个回合包括选择移动和进行移动两个部分。
    - 在选择移动的阶段，所有精灵都会进行选择，选择移动分为如下几个阶段
        1. 如果当前精灵在八个领接方向上都没有精灵，那么当前精灵不需要移动
        2. 存在四条移动规则，其中一条如下，如果当前精灵的北方、西北方和东北方的领接坐标处都没有精灵，那么当前精灵可以选择向北移动。每个精灵都会对四条移动规则进行判断，如果存在规则允许移动时，选择停止，后续规则不再判断。
        3. 如果对四条规则都完成了判断，同时也没有可选的移动目标，那么当前精灵不移动。
    - 在移动阶段，所有精灵一起移动，如果某一个坐标是两个及以上精灵的移动目的地，那么这些精灵都不移动。
    - 所有的精灵完成移动之后，需要调整移动规则的次序。当前四条规则中最先被考虑的规则将被移动到规则列表的尾部。
- 构造 Ground 类代表地图的情况，可能不是一个好的名字。包含了 elves 即所有精灵坐标的 HashSet，以及四条规则。
- 在输入处理中构造初始 elves ，同时根据题意硬编码移动规则。
- 移动规则的保存，我是直接以字符串数组的形式三个检测规则和最终移动方向。例如 `["N".to_string(), "NE".to_string(), "NW".to_string(), "N".to_string()]`
- 具体的坐标变换，编写了 4 个函数，分别对应向北、向南、向西或者向东移动一步。而对于 `NE` 这样的移动，实际上就是执行向北和向西的函数。可以直接使用 `match` 来快速计算移动后的坐标，通过四个函数就可以覆盖八个方向的所有移动。
- 在选择移动目的坐标的时候，首先判断当前精灵的八个领接方向是否存在精灵，如果都不存在直接放入 `next_move: HashSet<Pos>`，其中的精灵在这个回合都不移动。
- 然后，我利用 `possible_move: HashMap<Pos, Vec<Pos>>` 保存中间结果，其中的键为根据规则确定的下一个坐标，而值则为在这一个回合开始时这个精灵的坐标，因为一个目的坐标可能会被多个精灵选择，所以是 Vec。
- 当所有精灵完成了选择时，遍历 `possible_move` ，如果值对应的数组长度为 1 ，说明目标坐标只有一个精灵选择，将键即新坐标加入 `next_move` ，否则值中的所有精灵不移动，坐标不变，将数组中所有的精灵坐标加入 `next_move` 。
- 完成移动后，将移动规则的第一个元素移除，将其放到移动规则列表的尾部。移动规则以 `Vec` 存储，使用 `remove(0)` 进行移除，再通过 `push` 压入尾部。
- 当完成十个回合时，需要计算第一部分的结果，首先遍历此时所有精灵的坐标，取的每个坐标的范围，确定长方形的边界，计算长方形的面积，然后减去精灵的数量，得到第一部分结果。
- 第二个部分需要计算再多少个回合之后，所有的精灵都不进行移动，构造 loop 循环，循环中记录当前回合数，同时执行一回合，在每一个回合中统计没有移动的精灵数量，如果等于总的精灵数量，返回真，表示所有的精灵不再移动，循环结束并返回回合数。
- 两个部分的解答都不困难，在移动的逻辑上要注意题目要求，我漏看了一个移动规则浪费了一些时间。两个部分的执行效率也都可以接受，在 release 版本第二个部分花费半秒左右，而 debug 版本则会花费 15 s 左右，虽然不能算得上是极快，但是我并没有什么意见。也许第二个部分也可以进行优化，但是那要求对于每两个精灵的位置进行具体的分析，也许会有效率的提升，但是每次依旧需要更新精灵的坐标，不能确定进一步的优化是否有必要。

## Day 22（半手动完成第二部分）

[cube](https://user-images.githubusercontent.com/15051530/209130206-73dfea30-9480-4866-9ed6-891ff4d3c20f.png)

- 依旧是一道模拟题，根据输入的地图和指令进行移动，确定最终位置。
- 输入包含两部分：地图和移动指令，使用 HashMap 存储地图，方便确定位置，构造哈希表的过程中记录最远位置（最远列加一），作为地图边界，地图起始坐标为 (0, 0)。
- 地图形状并不规则，所以不能删除空格。存在三种方块，空白方块代表不存在。开放方块代表存在并且可在其上移动的方块。墙方块代表存在，但不能在其上移动的方块。
- 构造 Movement 枚举类型存放移动指令，直走为 S(i32) ，右转为 R ，左转为 L。
- 第一个部分较容易，移动逻辑如下：
    - 确定地图第一行最早的开放方块，这个坐标是起始坐标，初始时移动方向为向右。
    - 根据指令进行移动，存在两种移动方式，直走和转弯。
    - 如果直走中遇到墙那么当前移动终止。如果直走进入了空白区域，那么就要从地图的另一侧继续直走，空白部分类似于虫洞，直接连接两侧非空白区域。
    - 转弯是在原地进行的，不会改变具体坐标。
- 根据这个逻辑，可以直接进行模拟。直走过程中只有当移动到的位置是墙或者开放，才计算移动距离，如果移动进入空白区域，那么继续移动，但不影响实际坐标，如果移动超过了地图边界（输入构成的长方形，包含空白方块），对当前坐标取边界的模数（余数），然后再继续移动。直到统计的移动距离等于当前指令指定的直走距离，或者撞到墙，直走停止。
- 令移动方向向右为 0 向下为 1 向左为 2 向上为 3，可以发现方向对应的值随着顺时针方向增大，逆时针方向减小，向右转即是顺时针方向改变，向左转即是逆时针方向，利用这个特点快速调整方向，当方向为 4 或 -1 的时候，进行判断跨越即可。 4 == 0 ， -1 == 3 。
- 因为使用 HashMap ，而且坐标的类型为 i32 ，所以不需要担心在 0 处的溢出，根据不同的方向构造不同的坐标变化函数，即可实现不同方向的移动。
- 当指令执行完成后，得到最终的坐标和朝向，进行密码计算即可解决第一部分。
- 第二个部分非常复杂，不规则的地图实际上是一个立方体的表面展开，立方体的六个面互相连接。所以当移动到空白区域时，需要根据立方体面的连接情况进行对应的移动。整体的思路其实是一致的，只不过在计算移动到边界时的下一个坐标较难。
- 我首先在纸上画了我的输入中的地图，然后对每个面进行编号，折叠立方体，观察每个面边的连接情况。我并没有总结出通用的规律，最后我决定直接将连接边的对应关系进行硬编码。
    - 已知立方体边长为 50
    - 以起点和终点的二元组记录所有不相连的边，总共有 14 条这样的边。记录方向是从左到右，从上到下。
    - 观察这些边之间的连接关系，确定每条边的对应边。
    在折叠之后，两条边的起点和终点并不都是从左到右或从上到下的，部分的重叠边是相反的。
    - 如果一条边位于一个面的右侧，那么移动到这个边并产生跨越的情况时，移动方向一定是向右的（不考虑顶点），如果另一条边是处于面的下侧，那么移动方向就会从向右边为向上，所以也要记录从一条边到另一条重叠边的方向变化情况。
    - 最后我手写生成了一个元组，其中第一个元素为起始边，第二个元素为目标边，第三个元素为移动方向的改变。
    - 根据这些信息，就可以计算跨越时坐标和移动方向的变化。
- 这个方法异常的繁复，对于每一条边的坐标都要确保正确，确保边是处于面之内的，同时每一条边的对应边的情况也要记录正确。然后进行坐标计算的时候也要考虑到两条边的方向问题。在这些细节上我都犯过错误，但是最后终于得到了一个提交通过的答案。
- 这个做法并不通用，我根据自己的输入构造了边的对应情况，边的方向。这并不是一个通用的程序，如果要实现通用的程序，我需要实现以下功能：
    - 输入的过程中检测边，确定边的范围
    - 计算边与边之间的对应关系，以及对应边的方向关系
    - 根据边在面上的位置确定移动方向的变化
- **通用的方法**
    - https://www.reddit.com/r/adventofcode/comments/zsct8w/comment/j184mn7

## Day 21

- 今天题目不难，这让我担忧明天的题目难度。题目并不难，但是如果想要能够解决通用化的输入，那实际上依旧非常复杂，今天的输入有以下几个假设：
    - 输入中只有一个猴子和 humn 相关
    - root 猴子的值和 humn 一定是单调的（和前一个假设其实是一样的）
    - 除法操作一定是整除（即不使用浮点数作为数据类型，使用整形如果用二分法暴力查找，会得到好几个结果）
- 输入为猴子的列表，根据题意可以直接抽象为算数表达式
- 部分猴子对应数字，部分猴子对应算数表达式（与两个其他的猴子相关联）
- 第一部分需要计算名为 root 的猴子最后的值
- 利用哈希表存储猴子名和猴子编号的对应关系，利用数组存储所有的猴子，数组索引与猴子编号相互对应。每一个猴子可能对应一个数，或者对应两个猴子和一个操作，其中对应猴子也通过编号记录。
- 利用哈希表和数组避免了构造树，通过编号访问猴子的属性，也没有增加计算的复杂度。
- 第一部分直接模拟，深度优先搜索，同时构建 memo 实现记忆化搜索，从哈希表中得到 root 猴子的编号，从这个编号开始搜索。搜索逻辑如下：
    - 如果当前猴子对应一个数，则直接返回这个数
    - 如果当前猴子在 memo 中已经存在，直接返回 memo 的结果
    - 如果当前猴子对应一个算数表达式，那么对表达式中的两个猴子进行同样的搜索并得到结果，再根据操作对两个猴子的结果进行计算。将当前猴子编号和计算结果存入 memo，返回计算结果
    - **环是不存在的**，如果存在环就意味着存在无法计算 root 猴子结果，那么至少对于和 root 相关的所有猴子而言，环是不存在的，不需要进行环的检测。
- 在运行 release 版本的程序时得到了错误的结果，通过 debug 版本可以发现存在越界的情况，因为我使用 i32 记录计算结果，而乘法则导致存在溢出，替换为 i64 后结果正确。
- 第二个部分看似比第一部分复杂不少，root 猴子不再对应一个数学运算，而是判断其相关联的两个猴子的数值是否相同，同时 humn 也不是猴子，而是做题的人（我），需要计算的就是当我喊出的数值使得 root 对应的两个猴子的数量相同。 
- **简单枚举得到第二部分结果**，通过第一部分我已经知道了计算数值较大，所以暴力测试 humn 的值是不现实的，所以我首先观察了规律，测试了一些 humn 的值。测试过程中发现两个猴子的值其中一个是不和 humn 相关的，同时另一个猴子的值则和 humn 成反比 humn 越大值越小。知道这一点之后，不断的增大 humn 的值，使得两个猴子的差值越小。最后通过这个方法可以确定 humn 的值，而且存在好几个，提交通过。测试的方法没有问题，但是如何通过程序直接计算呢？大致有两个思路。
- **二分查找** humn ，如果 humn 和两个猴子的值存在关联，而且相关性是单调的，那么就可以通过二分法查找结果。
- **求解方程**，只存在一个未知数 humn，也只有一条方程，计算出两个猴子的算数表达式，然后求解方程。
    - 题目的输入比较凑巧，所以和 root 相关的两个猴子中只有一个与 humn 相关，而且所有猴子中只有一个猴子是和 humn 相关的，可以进行简单的方程求解
    - 一个表达式为包含 humn 的 f
    - 另一个表达式为常数
    - 那么直接逆向求解 f 直到 humn 即可

## Day 20

- 今天的题目简单一些，但在实现过程中依旧因为细节问题导致无法得出正确的答案。
- 每一行输入均为一个数字，处理简单，但在具体的计算过程中可能要引入辅助数据结构。
- 题意分析：
    - 输入为数字列表，需要根据列表中数字的值移动数字
    - 移动可以向前也可以向后，移动的次数为数字的值
    - 列表成环，当数字移动到列表尾部或者头部还没完成移动，那就需要从另一侧继续移动，**环的特征如下**：
        - 起点和终点实际上是同一个位置，考虑输入长度为 3，将输入视为数组，输入为 [-1, 1, 0]， 数字 -1 左侧的数字为 0 ，右侧的数字为 1 ，所以如果 -1 左移一个位置，也就是说 -1 右侧的数字为 0 ，所以数字 -1 新的索引就是 1，而不是 2。
        - 同样的对于输入 [-1, 0, 1]，数字 1 要右移一个位置，那么它新的索引应该是 1 而不是 0。
        - 对于一般的数组例如 [0, 1, 2]，如果数组也成环，那么在数字 2 之后就是 0 ，在数字 0 之前就是 1 ，往往可以直接通过 index % length 求解越界时新的索引，但在这个题目中这样的计算方式是错误的。
        - 因为是移动数据，所以可以想象这时移动的数据已经被删除了，对于第一个例子输入就变成了 [1, 0]，可以想象这个时候 -1 和 0 处于同一个位置，然后再左移一个位置，也就是 1 和 0 的中间。具体的可以利用 index % (length - 1) 进行计算。
        - 在 Rust 中的 % 运算计算的余数，而非取模，所以 -2 % 10 的结果是 -2 而不是 8。所以在计算小于 0 的索引时，还要在加上 length - 1 得到取模的结果。
        - 可以直接使用 next.rem_euclid(length - 1)
            - 参考 https://github.com/wilkotom/AoC2022/blob/main/day20/src/main.rs
    - 移动数字的顺序是输入的顺序
- 题目并不复杂，输入数据量也不大，解决这个问题的最优数据结构应该是链表，但我对链表并不熟悉，所以第一想法就是直接使用 Vec 实现。
- Vec 数组实现
    - 考虑只移动一个数字，当前位置为 cur ，数值值为 offset ，输入数据长度为 length，那么下一个位置 next = cur + offset，但是这个位置可能超出数组的范围，所以要考虑以下几种可能。
    - 如果 next >= 0 同时 next < length 即 next 处于范围内，那么不需要考虑越界的情况，next 即是数字新的位置。
    - 如果 next < 0 ，那么实际上的索引就是 next % (length - 1) + length - 1
    - 如果 next >= lenght 那么索引值就是 next % (length - 1)
    - 确定了待移动数字的目标位置，那么就可以移动了，直接利用 Vec 中 remove 和 insert 的方法即可。
    - 如果仅仅使用 Vec<i64> 来存储数字，那么在移动过程中输入的顺序就被打乱了，移动也无法按照输入顺序进行，所以需要记录移动的顺序。
    - 输入数字是存在重复的，所以利用 Vec 的 position 方法是无法准确找到目标索引的。
    - 在第一部分，我使用 Vec<(i64, bool)> 来存储输入，其中 bool 表示当前数字是否移动，移动数据后修改为 false 。无论移动的数字如何移动，还没有移动的数字他们的相对顺序，和输入一定是相同的，所以可以遍历每一个数字，如果当前数字对应的 bool 值为真，移动当前数字同时当前遍历结束，否则判断下一个数字。如果输入的数字数量为 n ，那么这样的遍历要进行 n 次，所以这个方法的时间复杂度是 O(n^2)，这样就得到了第一部分的解。
        - 在计算目标位置时，我为了减少越界的情况，所以提前降低了 offset ，但是却没注意到修改了输入数据，导致后续的移动都产生错误，完全是自己的粗心大意。
    - 第二个部分，需要对所有的输入进行 10 次移动，很自然的我想使用 Vec<(i64, usize)> 来实现，其中第二个数据表示当前数据被移动几次，在匆匆编写代码之后发现结果错误，和第一部分不同，未移动的数字顺序是和输入一致的，也就是说只有在第一次才有这个特性，而第二次移动时这个特性就完全不存在了，所以记录移动次数是不行的。实际上这个位置只要记录输入的位置即可，同时在一次移动数据的过程中，初始时当前要移动第 1 个数字，遍历所有数字，找到位置为 1 的数字，进行移动。以此类推完成所有数据的移动。第二部分这个方法的时间复杂度是 O(10*n^2)。
    - 这个方法的时间复杂度实际上是 O(n^3) 因为 Vec 插入和删除的操作时间复杂度为 O(n)，这个方法并不好。
- 结果的计算，两个部分都需要在数字移动完成后得到的序列中找到值为 0 的数字位置 zero_index，从这个位置之后 1000、 2000 和 3000 的数字值，并累加。因为计算结果并不涉及移动，所以直接通过 (zero_index + offset)%length 计算即可。
- 不移动插入、删除数组的方法
    - 输入存放在一个 Vec<(i32, i32)> 中，其中第一个数据表示数字的值，第二个数据表示当前位置。
    - 每一次按照顺序遍历 Vec ，计算新的位置并更新。
    - 计算结果时依旧是确定 0 的位置，然后计算第后 1000 个数字的位置，再通过 Vec 中的 position 确定数字的值即可。
    - **这个方法是错误的，因为移动一个数字之后，数组中其他数字记录的位置是错误的，需要更新其他数字的位置**
    - 出乎我的意料的是这个方法的运行时间大概是上面方法的六倍，我猜测可能是因为输入数据的太小导致的。Vec 插入和删除的时间复杂度是 O(n - i)，然后第一个方法中每一次都要进行 n 次移动，每次移动最差的情况内层循环需要执行 n 次，所以最差的时间复杂度的确是 O(n^3) 。而这个方法中，一定需要进行 n 次移动，每次移动需要还需要遍历其他 n - 1 个数据，所以时间复杂度是 O(n^2)。可能不是这个原因？即是我加大数据量结果依旧类似。


## Day 19 (其他方法的实现？)

- 按照我的实现程序运行的非常缓慢，今天的题目是我最不喜欢的那种题目，自己的思路没问题，剪枝也不成体系，记忆化也好像是胡乱一通，只能不断的想如何剪枝。最后即使参考了他人的方法，程序依旧不理想，运行时间甚至要慢于一些 python 的实现。
    - 剪枝优化可能性？
    - 更好的哈希方法？
- 大致就是直接模拟过程，DFS，**同时利用 HashSet 保存已经访问过的状态，状态由三个部分构成，当前各个机器人的数量，当前各个矿物的数量，以及当前时间**。
    - 使用最符合大小的数据类型能够减少程序运行时间，特别是使用巨大的哈希表的时候，如果系统涉及 swap 的使用，那么如果减少了空间也就加快的运行速度。**替换 usize 为 u16 之后，程序的运行时间直接减半。**
- 然后利用 DFS 暴力计算结果，但是这样的求解空间巨大，基本上不可能完成计算。
- 对于任意一个状态（一分钟），最多有四种可能，构造四种机器人的一种，以及等待一分钟。如果时间限制为 t ，那么求解空间大小为 5^t。
- 为了求解最大的 geode 的数量，那么如果能够制作 geode 机器人，就直接选择这种可能，其他的可能性都不会使得结果最大。求解空间减少到了 4^t。
- 这个剪枝的方法就让我的程序运行速度稍微可以接受，但是对于第一部分的大量蓝图运行速度依旧缓慢，所以我进一步剪枝。
- 同样的考虑是否也能在确定能够制作 obsidian 机器人时而不考虑其他的可能呢？并没有理论依据能够支持这样的剪枝，但是我并想不到其他的办法了，所以我就这样做了，再次剪枝后求解空间减少到了 3^t ，这时一个蓝图，模拟 24 分钟的情况，运行时间大概是 15s，而且示例的输入结果也是正确的，所以我就用这个方法计算第一部分的结果，大概花了五分钟，没想到结果是正确的。
- 同样的我想把这个方法用在第二部分上，需要模拟的时间增加到了 32 分钟，虽然计算一个蓝图所需时间并非小时级别，但是对于示例的两个蓝图输入，这样剪枝得到的结果并不正确，两个蓝图一个正确一个错误。因为第二部分的输入只有三个蓝图，因为使用了记忆化的方法，以及极其复杂的记忆化键，所以对每一个蓝图都会产生巨大的内存占用，所以我将输入分为三个，直接在另一台内存较大的电脑上同时运行。虽然我知道这样的剪枝并不准确，但是最后依旧得到了正确的答案。
- 虽然得到了正确的答案，但是这个程序的运行依旧是痛苦的，所以还要搞明白有哪些可能的优化。
- 记录蓝图模拟过程中最大的 geode 数量。然后计算当前状态下，不考虑资源的限制，到时间限制为止每一分钟都制作一个 geode 机器人，最后得到的 geode 数量。如果这个数量不大于目前已经计算到的最大 geode 数量，那么当前状态的后续情况就不必考虑了。 这个剪枝能够极大的提升效率，虽然自己也想到了，但是在计算当前状态下最大的 geode 数量我却算错了。在看了其他人的实现之后，我又重新实现这个方法，最后对于 4^t 的求解空间，两个部分都能在一分钟内完成运行。
- 因为在任意一分钟，只能制作一个新的机器人。假设 geode 机器人需要 4 个 ore ，5 个 obsidian， 那么如果有 5 个 ore 机器人就会导致浪费（至少对于制作 geode 机器人而言），同样的最多也只需要 5 个 obsidian 机器人就能满足需求。所以可以计算每一个蓝图，在一分钟中，每一种矿物所需要的最大数量，最大数量其实就对应了 ore 、clay 和 obsidian 机器人的最大数量。在计算下一个可能的状态时，可以利用这个条件进一步剪枝。（这个方法的优化不明显，因为往往超出机器人数量限制的时候，依旧是搜索的底层，只能减少少量的可能）。
- 还可以减少当前状态中的矿物，如果当前状态中的矿物数量已经远超过最大可能需要（每一分钟最多需要矿物*剩余时间），那么可以减少矿物的数量，提高缓存的命中率，但是效果非常不明显，理由类似。
- 还有其他的优化方法，包括减少记忆化包含的值，提高命中率，后续再学吧。
- 参考：
    - https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0tlukf
    - 随机测试组合机器人序列，测试结果。https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0tjfrt
    - 30ms rust 没有额外内存空间，直接计算构造机器人的类型，求解空间会远小于 4^t https://www.reddit.com/r/adventofcode/comments/zpihwi/comment/j0tol2k/?utm_source=share&utm_medium=web2x&context=3


## Day 18（待补充并查集实现）

- 今天的题目是一道连通题，只不过当我写完我才意识到这一点。
- 输入处理很简单，每一行是一个三维的坐标，字符串处理即可。
    - 最初我构造了 Cube 抽象化坐标，但是最后发现使用三元组会更加方便。
- 第一部分：计算所有方块的非重叠面积，不考虑重叠时，总表面积为方块数乘 6。如果两个方块存在重叠，那么总的表面积减少 2 。
- 输入的方块是仅有一个坐标，也就是一个点，如果仅仅考虑一个点，其实也就无从考虑重叠，所以在解决的过程中，是将输入作为方块的一个顶点。
- 可以发现当两个方块的**输入顶点**距离为 1 的时候，两个方块的一个面存在重叠关系，当距离为 0 时两个方块完全重叠，当距离为 2 时两个方块存在一条边重叠，当距离等于 3 时两个方块存在一个顶点重叠，距离大于 3 时方块不存在任何重叠。
- 第一个部分只需要计算出两个方块存在面重叠的数量即可。令输入方块数量为 n ，那么总共需要判断 C(n, 2) 次，C 为组合函数。
- 第二部分：需要计算最后物体暴露在外的区域面积，我采取了间接的计算方法。相比于第一部分，第二部分中需要减去的面积是所有由方块围成的空气方块的表面积。
- 除去组合成的物体，存在两种类型的空气方块
    - 和大气构成的空气方块，也就是最后物体之外的空气方块
    - 物体内部的空气方块，这些方块不和大气相连，并构成一个个的空气包
- 物体内部的空气包（由空气方块构成），无论大小如何，构成其的空气方块中一定有与岩浆方块重叠的情况，这就是寻找空气方块的入手条件。
- 一个方块最多与六个方块存在重叠（距离为 1 ），那么可以首先计算出所有岩浆方块的六个重叠方块坐标，排除岩浆方块就是所有可能的空气方块，其中可能构成空气包，也可能于大气相连。 遍历这些空气方块，进行搜索。
- 需要构造访问表来记录每一个空气方块的访问情况，构造 HashMap，键为坐标，值为 0 或 1
    - 其中 0 表示对应坐标处的空气方块已经被判断，而且该空气方块连通大气。
    - 1 表示对应坐标处的空气方块已经被判断或者正在被判断，不能确定该空气方块是否连通大气。
- 搜索逻辑如下，
- 当前空气方块为 A ，如果 A 在访问表中存在，那么无需判断，直接返回空即可。否则在访问表中置 A 的值为 1 表示正在搜索。
- 如果当前空气方块可能的六个重叠方块都是岩浆方块，这时就确定了一个空气包，这个空气包由一个空气方块构成，返回空气包大小为 1 ，包含空气方块的坐标。
- 如果当前空气方块的六个可能重叠方块不都是岩浆，那么需要进一步搜索，判断其中每一个重叠的空气方块：
    - 如果该空气方块已经在访问表中出现，而且值为 0 ，也就是说这个空气方块是连接到大气的，那么当前的空气方块也是连接到大气，没有进一步搜索的必要。在访问表中置当前空气方块为 0 ，返回空气包为空。
    - 如果该空气方块并未访问过，那么迭代的进行搜索，得到子空气包，如果子空气包大小为 0， 说明该空气方块是连接大气的，于是空气方块 A 也是连接大气，没有必要继续搜索。在访问表中置空气方块 A 为 0 ，返回空气包为空。
    - 如果所有重叠的空气方块都已完成搜索，但当前空气包大小为 0 ，也就是说当前空气方块连通大气，在访问表中置 A 为 0
- 在计算可能的重叠方块时，计算出的方块可能是与大气相连的，这时这个方块并未被访问，如果不加以控制，可能会无限制的搜索，所以需要增加边界，确定搜索停止的情况。
- 计算输入岩浆方块的范围，计算每个方向上的最大和最小值，最后得到两个坐标，岩浆方块构成的物体一定是处于由这两个坐标构成的六面体中（两个坐标处于对角线，这个六面体的每一个面和xyz三个面都垂直或平行），所以如果方块不处于这个六面体中，那么这个方块一定与大气相连。判断一个坐标是否处于一个六面体中，只需要计算这个坐标和得到的两个最小坐标和最大坐标的距离和是否与最小坐标和最大坐标之间的距离相同即可。通过这个方法可以规避死循环，同时也避免了大量重复计算。
- 在得到了构成每一个空气包的所有空气方块坐标后，计算这个空气包的表面积，因为每一个空气包全都有空气方块构成，不存在空洞，所以表面积实际上就是暴露的面积，这个部分的面积正是所需要减去的部分，可以利用第一部分的方法计算。
- 方法说明：
    - 这个方法可以算是 DFS 也可以算是 DP 
    - DFS 就是从可能的空气方块开始搜索，直到完成对这个空气方块所在的空气包搜索或者确认该空气方块连接大气停止。
    - DP 的实现则是自顶向下的迭代搜索。
        - 令 s 为可能的空气方块，s 未被搜索，同时 A(s) 表示所有与 s 存在一个面重叠的空气方块，同时 F(s) 表示 s 所在的空气包大小，转移方程如下：
        - 当 k 属于 A(s) 且 F(k) 都不为空时， F(s) = sum(F(k)) + s, 
        - 当 k 属于 A(s) 且存在 F(k) 为空时， F(s) 为空
- 过程中我一直以为自己用的是 DFS 但是看来好像 DP 的解释更加合理，当然这并不重要。完成之后我也意识到其实这也是一道并查集的题目。
- **并查集**
    - 计算岩浆的边界，得到范围内所有的坐标
    - 如果两个距离为 1 的坐标都为岩浆和空气，则这两个坐标间存在一条边
    - 和边界之外相连的坐标同时为空气，那么这个坐标和大气相连
    - 计算出所有空气包的坐标
    - 最后利用第一部分的方法，计算岩浆的表面积，再计算出每一个空气包（集）的面积，最后相减得到所求面积。
- **Flood fill 直接求解暴露面积 BFS / 自底向上的 DP**
    - 从边界最小处开始（0, 0, 0），依旧是判断六个方向上，如果某一方向方向上的方块为岩浆，说明存在一个面与岩浆重叠，面积加一
    - 如果不为岩浆，需要继续搜索，将该方向的坐标加入队列
    - 直到队列为空，搜索完成。
    - 如果使用队列那么就是 BFS ，如果使用栈那么就是 DFS
    - **同样要引入访问表确保不会出现死循环**
    - **因为是直接计算表面积，所以边界需要比岩浆的最大位置再大 1 。**
    - 这个方法运行速度最快，大致上和第一部分一样快，比我自己的方法也要快不少。
    - 参考：https://www.reddit.com/r/adventofcode/comments/zoqhvy/comment/j0oo08q

## Day 17(待补充)

- 第一部分直接模拟实现，第二部分数据量巨大，无论采用何种数据结构总是都是不可能急剧减少运行时间，所以猜测是存在规律的，每年都有这样的题目。不出意外的，我依旧没能找到规律，最后只好去 [r/adventofcode](https://www.reddit.com/r/adventofcode/comments/znykq2/2022_day_17_solutions/) 寻找参考。
- 如果存在规律，需要确定如下问题：
    - 因什么特征而有规律
    - 如何利用规律加快运算
- 第二部分参考：
    - https://www.reddit.com/r/adventofcode/comments/znykq2/comment/j0kc9qp
    - https://www.reddit.com/r/adventofcode/comments/znykq2/comment/j0kvtwd

## Day 16(待补充)

- 加权图
- 阀门的打开顺序有两个决定因素
    - 阀门的流速（正比，流速较快的越早打开越好）
    - 当前位置到目标阀门的距离（反比，较近的越早打开越好）
- 第一个阀门流速为 a ，在第 m 分钟打开，总压力为 0
- 第二个阀门流速为 b ，在第 n 分钟打开，总压力为 (n - m) * a
    - n - m 为第一个和第二个阀门的距离
- 第三个阀门流速为 c , 在第 k 分钟打开，总压力为 (n - m) * a + (k - n) * (a + b)
    - k - n 为第二个和第三个阀门的距离
- 需要计算两个阀门之间的最短距离（时间）
    - 我又用 DFS 来寻找最短路了，使用 BFS 之后完成了第一部分
- 第二个部分
    - 我和大象同时都打开阀门
- **优化代码永远比等效率差的代码完成执行要节省时间**

## Day 15

- 每一行输入包含一个传感器和其对应的最近的信标的坐标位置
- 利用 regex 和 lazy_static 通过正则表达式提取坐标
- 一个传感器存在三个重要的属性
    1. 传感器的坐标 `coord`
    2. 最近信标的坐标 `beacon`
    3. 传感器和最近信标的距离 `closest_dis`
- 根据这三个属性构造 `Sensor`
- 对于任意一个坐标，如果这个坐标和某一个传感器的距离小于等于传感器记录的最近距离，同时这个坐标也不是这个传感器记录的最近信标，那么这个坐标上一定不可能是信标
- 依照这个逻辑，可以通过排除法确定某一个坐标上是否是信标
- 输入的坐标值较大，而且并不需要记录具体的网格信息，所以既不需要使用 Vec 也不需要使用 HashMap
- 从输入中取的传感器列表，可以计算出地图的最大范围。x 坐标的最小值是所有传感器 x 轴坐标加上传感器和最近信标距离中的最小值，min{coord.x + closest_dis} 。可以确定 x 和 y 轴的范围。
- 第一个部分限定了网格区域，只考虑某一个 y 轴坐标，也就是一条线上的所有坐标。遍历这条线上所有的坐标，如果该坐标在任意的传感器计算中判断不可能是信标，那么该坐标满足条件，保留。最后对不可能是信标的坐标进行计数。这个部分的运行时间就达到了 2s ，可以说是非常慢的解法了。
- 第二个部分需要寻找一个没有被任何传感器检测到的信标，这个信标和所有传感器的距离都大于传感器的最近信标距离。信标的坐标在 (0, 0) 到 (4000000, 4000000) 的区域中。对于任意坐标，如果该坐标对于所有的传感器都有可能是信标，那么这个坐标就是要求的信标位置。
- 如果依旧同第一部分一样使用排除法，那么需要遍历 4000000^2 个坐标，用第一个部分的时间估算，**大概需要 5000 个小时才能排除所有的坐标。**
- 考虑坐标 (x,y) 和传感器 S，其中传感器坐标为 (a, b)，最近距离为 r 。令该坐标和传感器 S 的距离小于 r ，也就是说该坐标不可能是信标。传感器 S 划定了一个区域，这个区域中任何坐标到传感器的距离都不大于 r ，在网格上看起来就像是一个菱形（旋转 45 度的正方形）。即此时 (x, y) 落在这个菱形中，假设在当前行中遍历是从左到右的（x增加，y不变）。可以发现这个菱形和遍历过程中所有坐标相较与一个线段，这个线段上所有的坐标都处于菱形中，当然 (x, y) 也落在这个线段中，线段上的所有坐标都不需要再计算。设线段右端点坐标为 (x1, y)，那么就有：
    - |x1 - a| + |y - b| = r
    - |x1 - a| = r - |y - b|
    - x1 = r - |y - b| + a 或
    - x1 = -r + |y - b| + a
    - 两者取较大作为结果
- 即 x1 = max{r - |y - b| + a, -r + |y - b| + a}
- 通过这个方式，暴力的方法中每一次 x 的值只能增加一，但是经过这个优化，可以将 x 的值更新为 x1 + 1。在每一个 y 值的遍历（同一行）中，实际上只需要判断少量的坐标即可。
- 不能再通过这个方式加快 y 轴的遍历，因为这个方法实际上只确定了某一行上所有的坐标是否满足条件，并不能对 y 进行跳跃，所以 y 轴依旧只能一行一行的遍历。
- 在使用这个优化后运行时间在 5s 左右，也就是在每一行的遍历中，的确只计算了少数的坐标。
- 我发现按照传感器检测范围的最小值对传感器进行排序，也可以加速程序的运行，第二个部分的运行时间缩短到 3.5s 左右。在计算坐标是否落入传感器检测范围时，如果刚开始计算的传感器是距离较近的，那么就更有可能找到满足条件的传感器，也就能减少所需计算传感器的数量。

### Interval

参考链接：
- [Reddit/EVQLVE](https://www.reddit.com/r/adventofcode/comments/zmcn64/comment/j0anz77)

思路：
- 一个传感器由多个区间组成，对于给定的 y 值，可以确定传感器在这个线上的区间。
- 第一个部分计算出所有传感器在 y = 20 上的所有区间，合并区间，最终得到不重叠的区间列表，长度即是结果。
- 第二个部分对每一个 y 值都进行区间列表计算，如果得到的区间列表为长度为 2 此时即找到所求坐标，两个区间列表之中的坐标即是结果。
- 第二个部分中，题目保证在所求范围内仅有一个结果，即除了结果处，其余 y 值上合并区间后仅有一个区间，可以记录每一次合并区间时重叠区域的大小来对 y 值进行跳过。合并区间列表时，如果两个区间的重叠区域大小为 L ，因为传感器的检测范围是一个菱形（旋转的正方形），L 可以看作是一个小正方形的对角线，L / 2 即是从当前 y 开始，向下所有的部分都会重叠，后续可以不再判断。在合并过程中计算每一个重叠区域大小 L 的最小值，得到的结果的一半即是所有重叠区域在 y 轴方向上的小区间。于是原本需要每次对 y 加一，加速为 y + min{L / 2 + 1}，进一步加快程序运行。
- 这个方法运行时间：
    - Part1（Debug）为 45.542µs
    - Part2 (Debug) 为 1.35s
    - Part1（Release）为 5.75µs
    - Part2 (Release) 为 82.974166ms

### 两个方法对比

- 可以发现在第一个部分的运行时上新的方法提速明显，这很好理解，但第二个部分上区间法的优化就不明显了，这是因为区间法实际上是我自己实现的一种优化，当然区间法也更好理解。
- 自己的方法中，对坐标 (x, y)，实际上计算了某一传感器的区间的右边界，然后再在这个基础上计算下一个传感器的区间右边界，可以理解成是在计算的过程中合并了区间，而不是像第二个方法中直接计算所有的区间。其实我应该在发现对传感器进行排序能优化程序运行速度的时候就想到区间法的，比较在 interval 的算法题里，对区间列表进行排序总是必要的。

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