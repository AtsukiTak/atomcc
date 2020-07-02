mod generator;
mod subroutine;

pub use generator::Generator;

use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// GlobalにUniqueな数値を取得する。
/// GlobalにUniqueなラベルを生成するのに使用する
pub fn get_unique_num() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}
