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
|100|100|17,568,517 |7,186,489 |40.91%|
|100|500|30,231,574 |21,794,089 |72.09%|
|100|1000|30,697,446 |12,443,383 |40.54%|
|100|2000|33,844,956 |16,100,597 |47.57%|
|100|5000|33,456,229 |15,676,242 |46.86%|
|1000|100|30,809,995 |22,544,356 |73.17%|
|1000|500|34,041,163 |25,164,830 |73.92%|
|1000|1000|31,695,118 |23,580,345 |74.40%|
|1000|2000|35,061,147 |23,615,985 |67.36%|
|1000|5000|35,093,482 |24,367,149 |69.43%|

