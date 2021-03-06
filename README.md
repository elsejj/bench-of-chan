# 问题

给 N 个 worker 投递 M 个 event，这 N 个 worker 的处理逻辑运行在不同的用户态线程中，以此来测试队列（chan或是channel）以及协程调度的性能。

# 说明

worker 通过一个 event 输入队列来得到输入事件，它从该队列中重复的读取事件，直到收到结束事件，每收到一个事件，累加一个全局的原子（atmoic）计数器，当该计数器到达 M * N 时，通知所有任务完成。

事件为指定大小的字节数组。

- go 自带 `goroutine` 和 `chan`，所以每个 worker 运行在自己的 goroutine 中，队列则使用 chan
- rust 使用 [tokio](http://tokio.rs/) 作为调度运行时，每个 worker 是一个由 `tokio::spawn` 产生的[异步任务](https://docs.rs/tokio/1.2.0/tokio/task/index.html)，队列使用 `tokio::sync::mpsc::channel`

# 结果

结果是每秒的消息投递数目，详细的如下表所示

| worker | event | go         | rust       | diff (go/rust) |
| ------ | ----- | ---------- | ---------- | -------------- |
| 100    | 100   | 12,242,899 | 12,125,621 | 100.97%        |
| 1000   | 100   | 21,567,993 | 17,556,180 | 122.85%        |
| 10000  | 100   | 16,693,599 | 17,240,636 | 96.83%         |
| 100000 | 100   | 12,712,109 | 12,206,104 | 104.15%        |
| 100    | 500   | 14,670,070 | 14,088,079 | 104.13%        |
| 1000   | 500   | 22,997,811 | 20,110,447 | 114.36%        |
| 10000  | 500   | 22,335,358 | 19,899,927 | 112.24%        |
| 100000 | 500   | 13,986,726 | 13,038,232 | 107.27%        |
| 100    | 1000  | 15,853,639 | 13,998,936 | 113.25%        |
| 1000   | 1000  | 23,532,845 | 21,392,708 | 110.00%        |
| 10000  | 1000  | 22,455,318 | 20,192,903 | 111.20%        |
| 100000 | 1000  | 14,259,034 | 13,202,118 | 108.01%        |
| 100    | 2000  | 9,233,610  | 10,348,271 | 89.23%         |
| 1000   | 2000  | 21,938,990 | 20,358,866 | 107.76%        |
| 10000  | 2000  | 22,537,573 | 16,118,316 | 139.83%        |
| 100000 | 2000  | 14,730,508 | 12,903,106 | 114.16%        |
| 平均   |       | 17,609,255 | 15,923,778 | 109.76%        |