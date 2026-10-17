//! A supplied current-thread runtime is driven like the default one.

use std::sync::mpsc;
use std::time::Duration;

use async_compat::CompatExt;

/// A current-thread scheduler only makes progress while something drives it,
/// so without a thread holding it open the spawned task would never run.
#[test]
fn a_supplied_current_thread_runtime_is_driven() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    async_compat::set_fallback_runtime(rt).unwrap();

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let answer = futures::executor::block_on(
            async { tokio::spawn(async { 42 }).await.unwrap() }.compat(),
        );
        tx.send(answer).unwrap();
    });

    assert_eq!(rx.recv_timeout(Duration::from_secs(10)), Ok(42));
}
