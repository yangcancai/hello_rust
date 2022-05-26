use std::{
    alloc::{alloc, dealloc, Layout},
    ops::Deref,
    sync::atomic::Ordering,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        execute();
    }
}
#[repr(align(8))]
struct IVec([u8; 8], usize);
impl IVec {
    fn new(slice: &[u8]) -> IVec {
        let mut data = [0_u8; 8];
        // 内存分配大小和对齐方式
        let layout = Layout::from_size_align(slice.len(), 8).unwrap();
        // 分配内存
        unsafe {
            let ptr = alloc(layout);
            // 拷贝slice到新的内存地址
            std::ptr::copy_nonoverlapping(slice.as_ptr(), ptr, slice.len());
            // data存放ptr的地址
            // 'as _' 自动推导出类型
            // std::ptr::write_unaligned(data.as_mut_ptr() as _, ptr);
            std::ptr::write_unaligned(data.as_mut_ptr() as *mut *mut u8, ptr);
        }
        IVec(data, slice.len())
    }
    fn get(&self) -> &[u8] {
        self.as_ref()
    }
    /// 获取指针地址
    fn remote_ptr(&self) -> *const u8 {
        unsafe { std::ptr::read(self.0.as_ptr() as *const *const u8) }
    }
}
/// 解析,资源释放
impl Drop for IVec {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.1, 8).unwrap();

        std::sync::atomic::fence(Ordering::Acquire);
        unsafe {
            dealloc(self.remote_ptr() as *mut u8, layout);
        }
    }
}
/// Vec<u8>.into() 会执行IVec::from(Vec<u8>)
impl From<Vec<u8>> for IVec {
    fn from(slice: Vec<u8>) -> Self {
        IVec::new(&slice)
    }
}
/// 解引用
/// &IVec to &[u8]
impl Deref for IVec {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        unsafe {
            let data_ptr = self.remote_ptr();
            // 读取内存数据
            std::slice::from_raw_parts(data_ptr, self.1)
        }
    }
}
/// 引用
impl AsRef<[u8]> for IVec {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self
    }
}
pub fn execute() {
    let mut key = vec![];
    key.extend_from_slice(b"kkkkkkkkkkkkkkkkk");
    let b = key.clone();
    // from和into结果是一样的
    // into里面也是执行from
    //   let key_i: IVec = IVec::from(key);
    let key_i: IVec = key.into();
    assert_eq!(&b, key_i.get());
}
