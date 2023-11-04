use std::any::{Any, TypeId};
use std::collections::HashMap;



/// #### 한국어 </br>
/// 애플리케이션에서 사용하는 객체를 담고 있습니다. </br>
/// 각 타입의 객체는 하나만 저장될 수 있습니다. </br>
/// 
/// #### English (Translation) </br>
/// Contains objects used by the application. </br>
/// Only on object of each type can be stored. </br>
/// 
#[derive(Debug)]
pub struct Shared(HashMap<TypeId, Box<dyn Any>>);

impl Shared {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// #### 한국어 </br>
    /// 주어진 요소를 추가합니다. </br>
    /// 만약 주어진 요소가 이미 존재하는 경우 이전의 요소를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Adds the given elements. </br>
    /// If the given element already exists, it returns the previous element. </br>
    /// 
    #[inline]
    pub fn push<T: 'static>(&mut self, value: T) -> Option<T> {
        self.0.insert(TypeId::of::<T>(), Box::new(value))
            .map(|ptr| ptr.downcast().ok().unwrap())
            .map(|ptr| *ptr)
    }

    /// #### 한국어 </br>
    /// 해당 요소를 제거합니다. </br>
    /// 만약 해당 요소가 존재하지 않은 경우 `None`을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Removes that element. </br>
    /// If the element does not exist, it return `None`. </br>
    /// 
    #[inline]
    pub fn pop<T: 'static>(&mut self) -> Option<T> {
        self.0.remove(&TypeId::of::<T>())
            .map(|ptr| ptr.downcast().ok().unwrap())
            .map(|ptr| *ptr)
    }

    /// #### 한국어 </br>
    /// 해당 요소를 빌려옵니다. (reference) </br>
    /// 만약 해당 요소가 존재하지 않은 경우 `None`을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows that element. (reference) </br>
    /// If the element does not exist, it returns `None`. </br>
    /// 
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.0.get(&TypeId::of::<T>())
            .map(|ptr| ptr.downcast_ref().unwrap())
    }

    /// #### 한국어 </br>
    /// 해당 요소를 빌려옵니다. (mutable) </br>
    /// 만약 해당 요소가 존재하지 않은 경우 `None`을 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrows that element. (mutable) </br>
    /// If the element does not exist, it returns `None`. </br>
    /// 
    #[inline]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut(&TypeId::of::<T>())
            .map(|ptr| ptr.downcast_mut().unwrap())
    }
}
