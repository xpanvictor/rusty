use std::time::{Duration, Instant};
use futures::{join};
use marshal_async::{Runtime, yield_now};

/// Basic smoke test: spawns one task and verifies result
#[test]
fn spawns_and_returns_value() {
    let mut runtime = Runtime::new();

    let handle = runtime.spawner.spawn(async {
        println!("Hello from async task");
        42
    });

    runtime.run(); // drive executor until task completes
    let result = handle.join();
    assert_eq!(result, 42);
}

/// Tests multiple tasks completing concurrently
#[test]
fn runs_multiple_tasks_concurrently() {
    let mut runtime = Runtime::new();

    let h1 = runtime.spawner.spawn(async {
        println!("Task 1 start");
        yield_now().await;
        println!("Task 1 done");
        1
    });

    let h2 = runtime.spawner.spawn(async {
        println!("Task 2 start");
        yield_now().await;
        println!("Task 2 done");
        2
    });

    runtime.run();

    assert_eq!(h1.join(), 1);
    assert_eq!(h2.join(), 2);
}

/// Tests nested async blocks inside a single future
#[test]
fn handles_nested_awaits() {
    let mut runtime = Runtime::new();
    let val = 64;

    let nested_task = async move {
        async { println!("hello world, I'm async") }.await;
        async { println!("hello world, I'm nested") }.await;
        val
    };

    let res_handle = runtime.spawner.spawn(nested_task);
    runtime.run();
    let res = res_handle.join();
    assert_eq!(res, 64);
}

/// Tests join! macro runs tasks cooperatively (interleaved)
#[test]
fn executes_joined_tasks() {
    let mut runtime = Runtime::new();

    let slow = async {
        println!("slow start");
        for _ in 0..20 {
            yield_now().await;
            println!("slow yielding...");
        }
        println!("slow done");
        1
    };

    let fast = async {
        println!("fast start");
        for _ in 0..3 {
            yield_now().await;
            println!("fast yielding...");
        }
        println!("fast done");
        2
    };

    let combined = async {
        let (a, b) = join!(slow, fast);
        a + b
    };

    let res_handle = runtime.spawner.spawn(combined);
    runtime.run();
    let res = res_handle.join();
    assert_eq!(res, 3);
}

/// Tests block_on runs inline without needing explicit run()
#[test]
fn block_on_works_inline() {
    let mut runtime = Runtime::new();

    let output = runtime.block_on(async {
        yield_now().await;
        99
    });

    assert_eq!(output, 99);
}

/// Tests that the runtime stops automatically when all tasks are complete
#[test]
fn runtime_stops_when_no_active_tasks() {
    let mut runtime = Runtime::new();

    for i in 0..5 {
        runtime.spawner.spawn(async move {
            yield_now().await;
            println!("Task {i} done");
        });
    }

    let start = Instant::now();
    runtime.run();
    let elapsed = start.elapsed();

    // The runtime should stop quickly (no hang)
    assert!(elapsed < Duration::from_secs(2), "Runtime didn't stop in time");
    assert_eq!(runtime.active.load(std::sync::atomic::Ordering::SeqCst), 0);
}