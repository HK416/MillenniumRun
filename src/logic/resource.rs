use std::any::{Any, TypeId};
use std::collections::HashMap;



/// #### 한국어 </br>
/// 전역적으로 사용하는 게임 리소스를 저장합니다. </br>
/// 
/// #### English (Translation) </br>
/// Stores globally used game resources. </br>
/// 
#[derive(Default)]
pub struct Resources {
    map: HashMap<TypeId, Box<dyn Any>>,
}


impl Resources {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn insert<T: 'static>(&mut self, val: T) -> Option<T> {
        self.map.insert(TypeId::of::<T>(), Box::new(val))
            .map(|ptr| ptr.downcast().ok().unwrap())
            .map(|ptr| *ptr)
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map.remove(&TypeId::of::<T>())
            .map(|ptr| ptr.downcast().ok().unwrap())
            .map(|ptr| *ptr)
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())
            .map(|ptr| ptr.downcast_ref().unwrap())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut(&TypeId::of::<T>())
            .map(|ptr| ptr.downcast_mut().unwrap())
    }
}
