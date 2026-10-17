//! Supplying the fallback runtime with `set_fallback_runtime`.
//!
//! The fallback runtime is process-wide and can only be set once, so each test
//! binary for it holds a single test.

use async_compat::CompatExt;

/// Tasks spawned from inside `Compat` run on the supplied runtime, and since
/// that runtime is multi-threaded, `block_in_place` works in them even without
/// the `multi-thread` feature.
#[test]
fn spawned_tasks_run_on_the_supplied_runtime() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("supplied-runtime")
        .enable_all()
        .build()
        .unwrap();
    async_compat::set_fallback_runtime(rt).unwrap();

    // Deliberately outside any tokio context, so `Compat` falls back.
    let (name, answer) = futures::executor::block_on(
        async {
            tokio::spawn(async {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                let name = std::thread::current().name().map(str::to_owned);
                (name, tokio::task::block_in_place(|| 42))
            })
            .await
            .expect("spawned task panicked")
        }
        .compat(),
    );

    assert_eq!(name.as_deref(), Some("supplied-runtime"));
    assert_eq!(answer, 42);
}
