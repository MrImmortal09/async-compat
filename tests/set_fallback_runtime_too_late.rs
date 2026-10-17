//! `set_fallback_runtime` after the fallback runtime already exists.

use async_compat::CompatExt;

/// Once a `Compat` has fallen back, the runtime it used stays in place and a
/// later one is handed back rather than silently dropped.
#[test]
fn a_runtime_supplied_too_late_is_handed_back() {
    let spawned_on = || {
        futures::executor::block_on(
            async {
                tokio::spawn(async { std::thread::current().name().map(str::to_owned) })
                    .await
                    .unwrap()
            }
            .compat(),
        )
    };

    assert_eq!(spawned_on().as_deref(), Some("async-compat/tokio-1"));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .thread_name("too-late")
        .enable_all()
        .build()
        .unwrap();
    let rt = async_compat::set_fallback_runtime(rt).unwrap_err();
    assert_eq!(
        rt.handle().runtime_flavor(),
        tokio::runtime::RuntimeFlavor::MultiThread
    );

    assert_eq!(spawned_on().as_deref(), Some("async-compat/tokio-1"));
}
