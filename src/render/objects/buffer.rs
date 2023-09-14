use std::mem;
use std::cmp;
use std::hash::{Hash, Hasher};

use crate::render::identifier::{
    IDHandle,
    generate_id_handle,
};



/// #### 한국어 </br>
/// 버퍼를 생성하고 관리하는 구조체 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a structure that creates and manages buffer. </br>
/// 
#[derive(Debug, Default, PartialEq, Eq)]
pub struct BufferPool(Vec<BufferObj>);

impl BufferPool {
    /// #### 한국어 </br>
    /// 새로운 버터 관리 풀을 생성합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Create a new buffer management pool. </br>
    /// 
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// #### 한국어 </br>
    /// 주어진 [`BufferDescriptor`](wgpu::BufferDescriptor)로 버퍼를 생성하고,
    /// 생성된 버퍼의 식별자를 반환합니다. </br>
    /// 이 함수는 풀의 요소가 식별자를 기준으로 오름차순 정렬되어있음을 보장합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates a buffer with the given [`BufferDescriptor`](wgpu::BufferDescriptor),
    /// and returns the identifier of the created buffer. </br>
    /// This function ensures that the elements in the pool
    /// are sorted by identifier in ascending order. </br>
    /// 
    pub fn insert<'a>(
        &mut self, 
        device: &wgpu::Device,
        desc: &wgpu::BufferDescriptor<'a>
    ) -> IDHandle {
        let id = generate_id_handle();
        self.0.push(BufferObj { 
            id: id.clone(), 
            buffer: device.create_buffer(desc) 
        });
        return id;
    }

    /// #### 한국어 </br>
    /// 주어진 [`BufferInitDescriptor`](wgpu::util::BufferInitDescriptor)로 버퍼를 생성하고,
    /// 생성된 버퍼의 식별자를 반환합니다. </br>
    /// 이 함수는 풀의 요소가 식별자를 기준으로 오름차순 정렬되어있음을 보장합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Creates a buffer with the given [`BufferInitDescriptor`](wgpu::util::BufferInitDescriptor),
    /// and returns the identifier of the created buffer. </br>
    /// This function ensures that the elements in the pool
    /// are sorted by identifier in ascending order. </br>
    /// 
    pub fn insert_with_init<'a>(
        &mut self,
        device: &wgpu::Device,
        desc: &wgpu::util::BufferInitDescriptor<'a>
    ) -> IDHandle {
        use wgpu::util::DeviceExt;
        let id = generate_id_handle();
        self.0.push(BufferObj { 
            id: id.clone(), 
            buffer: device.create_buffer_init(desc) 
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
    pub fn get(&self, id: &IDHandle) -> Option<&BufferObj> {
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
    pub fn get_mut(&mut self, id: &IDHandle) -> Option<&mut BufferObj> {
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
/// 게임에서 사용되는 버퍼 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the buffer used in the game. </br>
/// 
#[derive(Debug)]
pub struct BufferObj {
    id: IDHandle,
    buffer: wgpu::Buffer,
}

impl BufferObj {
    /// #### 한국어 </br>
    /// `BufferObj`의 식별자를 대여합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Borrow the identifier of `BufferObj`. </br>
    /// 
    #[inline]
    pub fn ref_id(&self) -> &IDHandle {
        &self.id
    }
}

impl AsRef<wgpu::Buffer> for BufferObj {
    #[inline]
    fn as_ref(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

impl Ord for BufferObj {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.ref_id().cmp(other.ref_id())
    }
}

impl PartialOrd<Self> for BufferObj {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.ref_id().partial_cmp(other.ref_id())
    }
}

impl Eq for BufferObj { }

impl PartialEq<Self> for BufferObj {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ref_id().eq(other.ref_id())
    }
}

impl Hash for BufferObj {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ref_id().hash(state)
    }
}


