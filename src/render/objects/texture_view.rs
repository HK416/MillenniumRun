use std::cmp;
use std::mem;
use std::hash::{Hash, Hasher};

use crate::render::{
    objects::TextureObj,
    identifier::{
        IDHandle,
        WeakIDHandle,
        generate_id_handle,
    },
};



/// #### 한국어 </br>
/// 텍스쳐 뷰를 생성하고 관리하는 구조체입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that creates and manages texture view. </br>
/// 
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TextureViewPool(Vec<TextureViewObj>);

impl TextureViewPool {
    /// #### 한국어 </br>
    /// 새로운 텍스쳐 뷰 관리 풀을 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Create a new texture view management pool. </br>
    /// 
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// #### 한국어 </br>
    /// 주어진 [`TextureViewDescriptor`](wgpu::TextureViewDescriptor)로 텍스쳐 뷰를 생성하고,
    /// 생성된 텍스쳐 뷰의 식별자를 반환합니다. </br>
    /// 이 함수는 풀의 요소가 식별자를 기준으로 오름차순 정렬되어있음을 보장합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates a texture view with the given [`TextureViewDescriptor`](wgpu::TextureViewDescriptor),
    /// and returns the identifier of the created texture view. </br>
    /// This function ensures that the elements in the pool
    /// are sorted by identifier in ascending order. </br>
    /// 
    #[inline]
    pub fn insert<'a>(
        &mut self,
        texture: &TextureObj,
        desc: &wgpu::TextureViewDescriptor<'a>
    ) -> IDHandle {
        let id = generate_id_handle();
        self.0.push(TextureViewObj { 
            id: id.clone(), 
            view: texture.as_ref().create_view(desc)
        });
        return id;
    }

    /// #### 한국어 </br>
    /// 풀에 있는 요소들 중에 주어진 식별자와 일치하는 요소가 있는지 찾습니다. </br>
    /// 주어진 식별자와 일치하는 요소를 찾은 경우 인덱스를 반환합니다. </br>
    /// 최대 `O(log(n))`의 시간이 소요됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Find if any of the elements in the pool match a given identifier. </br>
    /// If an element matching the given identifier is found, it returns the index. </br>
    /// It takes at most `O(log(n))` time. </br>
    /// 
    #[inline]
    fn binary_search(&self, id: &IDHandle) -> Option<usize> {
        self.0.binary_search_by(|item| item.ref_id().cmp(id)).ok()
    }

    /// #### 한국어 </br>
    /// 풀에 있는 요소들 중에 주어진 식별자와 일치하는 요소가 있는 경우 빌려옵니다. <b>(reference ver)</b></br>
    /// 최대 `O(log(n))`의 시간이 소요됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// If any element in the pool matches the given identifier, it is borrowed. <b>(reference ver)</b></br>
    /// It takes at most `O(log(n))` time. </br>
    /// 
    #[inline]
    pub fn get(&self, id: &IDHandle) -> Option<&TextureViewObj> {
        self.binary_search(id).map(|idx| self.0.get(idx))?
    }

    /// #### 한국어 </br>
    /// 풀에 있는 요소들 중에 주어진 식별자와 일치하는 요소가 있는 경우 빌려옵니다. <b>(mutable ver)</b></br>
    /// 최대 `O(log(n))`의 시간이 소요됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// If any element in the pool matches the given identifier, it is borrowed. <b>(mutable ver)</b></br>
    /// It takes at most `O(log(n))` time. </br>
    /// 
    #[inline]
    pub fn get_mut(&mut self, id: &IDHandle) -> Option<&mut TextureViewObj> {
        self.binary_search(id).map(|idx| self.0.get_mut(idx))?
    }

    /// #### 한국어 </br>
    /// 풀에 있는 요소들 중에 사용되지 않는 요소들을 정리합니다. </br>
    /// 최대 `O(n)`의 시간이 소요됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Clean up unused elements in the pool. </br>
    /// It takes at most `O(n)` time. </br>
    /// 
    #[inline]
    pub fn cleanup_unused_items(&mut self) {
        self.0 = mem::take(&mut self.0).into_iter()
            .filter(|it| it.ref_id().strong_count() > 1)
            .collect();
    }
}



/// #### 한국어 </br>
/// 게임에서 사용되는 텍스쳐 뷰 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the texture view used in the game. </br>
/// 
#[derive(Debug)]
pub struct TextureViewObj {
    id: IDHandle,
    view: wgpu::TextureView,
}

impl TextureViewObj {
    /// #### 한국어 </br>
    /// `TextureViewObj`의 식별자를 대여합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrow the identifier of `TextureViewObj`. </br>
    /// 
    #[inline]
    pub fn ref_id(&self) -> &IDHandle {
        &self.id
    }
}

impl AsRef<wgpu::TextureView> for TextureViewObj {
    #[inline]
    fn as_ref(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl Ord for TextureViewObj {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.ref_id().cmp(other.ref_id())
    }
}

impl PartialOrd<Self> for TextureViewObj {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.ref_id().partial_cmp(other.ref_id())
    }
}

impl Eq for TextureViewObj { }

impl PartialEq<Self> for TextureViewObj {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ref_id().eq(other.ref_id())
    }
}

impl Hash for TextureViewObj {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ref_id().hash(state)
    }
}
