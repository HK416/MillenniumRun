use std::sync::{Arc, Weak};



/// #### 한국어 </br>
/// 모든 렌더링 컨텍스트에 대한 핸들입니다.
/// 핸들은 고유의 변경 불가능한 식별자를 가지고 있습니다. </br>
/// 핸들을 복제하여 여러 스레드에서 공유하여 사용할 수 있습니다. </br>
/// 핸들을 복제할 때 내부 참조 횟수를 증가시키고 싶지 않은 경우 `IDHandle::downgrade`를 사용해야 합니다. </br>
/// 
/// #### English (Translation) </br>
/// A handle to any rendering context. </br>
/// A handle has a unique, immutable identifier. </br>
/// By duplicating the handle, it can be shared and used by multiple threads. </br>
/// You should use `IDHandle::downgrade` if you do not want to increment the internal reference count when duplicating handles. </br>
/// 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IDHandle(Arc<u64>);

impl IDHandle {
    /// #### 한국어 </br>
    /// 핸들의 식별자를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the identifier of handle. </br>
    /// 
    #[inline]
    pub fn id(&self) -> u64 {
        self.0.as_ref().clone()
    }

    /// #### 한국어 </br>
    /// 핸들의 참조 횟수를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Returns the reference count of handle. </br>
    /// 
    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    /// #### 한국어 </br>
    /// 내부 참조 횟수를 증가시키지 않고 핸들을 복제합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Duplicate the handle without incrementing the internal reference count. </br>
    /// 
    #[inline]
    pub fn downgrade(this: &Self) -> WeakIDHandle {
        WeakIDHandle(Arc::downgrade(&this.0))
    }
}



/// #### 한국어 </br>
/// 모든 렌더링 컨텍스트에 대한 핸들입니다. </br>
/// 핸들의 식별자를 가져오려면 `WeakIDHandle::upgrade`함수를 사용하여 `IDHandle`로 업그레이드 해야 합니다. </br>
/// 핸들을 복제하여 여러 스레드에서 공유하여 사용할 수 있습니다. </br>
/// 이 핸들은 복제할 때 내부 참조 힛수를 증가시키지 않습니다. 따라서 원본 핸들이 삭제된 상태일 수 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// A handle to any rendering context. </br>
/// To get the identifier of handle, you need to upgrade it to a `IDHandle` using the function `WeakIDHandle::upgrade`. </br>
/// By duplicating the handle, it can be shared and used by multiple threads. </br>
/// This handle does not increment its internal reference count when cloned. Therefore, the original handle may be in a deleted state. </br>
/// 
#[derive(Debug, Clone)]
pub struct WeakIDHandle(Weak<u64>);

impl WeakIDHandle {
    /// #### 한국어 </br>
    /// `IDHandle`로 업그레이드하려고 시도합니다. </br>
    /// 원본 `IDHandle`이 이미 삭제된 경우 `None`을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Attempting to upgrade to `IDHandle`. </br>
    /// Returns `None` if the original `IDHandle` has already been deleted. </br>
    /// 
    #[inline]
    pub fn upgrade(&self) -> Option<IDHandle> {
        Some(IDHandle(self.0.upgrade()?))
    }
}


/// #### 한국어 </br>
/// 새로운 `IDHandle`을 생성합니다. </br>
/// `1`에서 `18,446,744,073,709,551,614`까지의 id 값을 할당받습니다. </br>
/// 이 범위를 초과할 경우 프로그램 실행을 중단합니다. </br>
/// id번호 할당은 원자적으로 동작합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a new `IDHandle`. </br>
/// It is assigned an id value from `1` to `18,446,744,073,709,551,614`. </br>
/// If this range is exceeded, abort program execution. </br>
/// The id number assignment works atomically. </br>
/// 
pub(super) fn generate_id_handle() -> IDHandle {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNT: AtomicU64 = AtomicU64::new(1);
    let num = COUNT.fetch_add(1, Ordering::AcqRel);
    assert_ne!(num, u64::MAX, "The maximum id value that can be created has been exceeded.");
    IDHandle(num.into())
}