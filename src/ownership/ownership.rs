pub fn ownership_mod() {
  println!("----- 所有权 ownership -----");
  // rust的核心特性就是所有权
  // 所有的程序在运行时都必须管理他们使用计算机内存的方式
  // 有些语言有垃圾回收机制，在程序运行时，他们会不断地寻找不再使用的内存
  // 在其他语言中，程序员必须显示地分配和释放内存
  // rust采用了第三种方式
  // 内存是通过一个所有权系统来管理的，其中包含一组编译器在编译时检查的规则
  // 当程序运行时，所有权特性不会减慢程序的 运行速度。
  // stack vs heap
  // 在像rust这样的系统级编程语言里面，一个值是在stack上还是在heap上对语言的行为和你为什么做某些决定是有更大的影响的
  // 在代码运行的时候，stack和heap都是可用的内存，但是它们的结构很不相同。
  // stack： 按照值得接收顺序来存储，按照相反的顺序将它们移除； 后进先出 LIFO
  // 添加数据叫做压入栈
  // 移除数据叫做弹出栈
  // 所有存储在stack上的数据必须拥有已知的固定的大小
  // - 编译时大小未知的数据或运行时大小可能发生变化的数据必须存放在heap上

  // heap 的内存组织性差一些
  // 当把 数据放入heap时，你会请求一定数量的空间
  // 操作系统在heap里找到一块足够大的空间，把他标记为在用，在、并返回一个指针，这个指针就是这个空间的地址
  // 这个过程叫做heap上分配

  // 因为指针是已知固定大小，可以把指针放到stack上
  // 但如果想访问实际的数据，必须使用指针来定位
  // 把数据压到stack上比分配到heap中要快的多
  // - 因为操作系统不需要去寻找存放数据的存储空间，那个位置永远都在stack的顶端

  // 在heap中存放数据需要更多的工作
  // - 首先要寻找足够大的空间来存放数据，然后做好记录方便下次分配
  // 访问heap中数据要比访问stack中的数据慢，因为需要指针才能找到heap中的数据

  // 对于现代的处理器来说，由于缓存的缘故，如果指令在内存中跳转的次数越少，则速度的越快

  // 当代吗调用函数时，值被传入到函数，函数本地的变量被压到stack上，当函数结束后，将值从stack弹出

  // 所有权存在的原因
  // 所有权解决的问题：
  // 1、跟踪代码的哪些部分正在使用heap的哪些数据
  // 2、最小化heap上的重复数据
  // 3、清理heap上未使用的数据以避免空间不足
  // 一旦你懂的了所有权，那么就不需要经常去想stack 或 heap了
  // 但是知道管理heap数据是所有权的原因，这有助于驾驶他为什么会这样工作

  // 所有权规则
  // 1、每个值都有一个变量，这个变量是该值的所有者
  // 2、每个值同时只能有一个所有者
  // 3、当所有者超出作用域（scope）时，该值将被删除
  println!("----- end -----");
  println!();
}