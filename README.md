# 项目名称
- **java-threadpool**: 这是一个跟java ThreadPoolExecutor线程池 使用方式基本相同的线程池


## 快速开始
使用 java-threadpool的示例:

    // 创建线程池
    let mut thre = ThreadPool::new(2, 5, 20);
    let mut vc = Vec::<Future>::new();

    for i in 0..10 {
        // 向线程池内提交闭包（执行的逻辑）
        let future = thre.executor(|| {
            thread::sleep(Duration::from_secs(5));
            println!("HELLO WORLD");

        });
        // 将返回的 Future 对象存储在vec内
        vc.push(future);

    }

    for v in vc{
        // 阻塞线程，等待提交的闭包执行完成
        v.get();
    }
    // 关闭线程池
    thre.shutdown();

## ThreadPool::new() 参数的含义是：

- **core_pool_size**: 核心线程数，就是常驻线程数。
- **maximum_pool_size**: 最大线程数，这个参数必须比核心线程数大。
- **maximum_queue**: 等待执行线程的队列长度。最大有多少个任务等待，
                   如果线程提交数量超过 core_pool_size + maximum_pool_size +maximum_queue
    那么线程会在 thre.executor 方法处阻塞，等待队列内空出新的位置。
