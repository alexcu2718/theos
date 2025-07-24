#![no_std]
//experimental no std library
//THSI IS EXTREMELY SKETCHY
// MAIN PLAN IS TO BUILD A NO-STD LIBRARY TO USE IN MY TOYOS

use core::mem::{self, MaybeUninit};
use core::ops::{Index, IndexMut};

use core::slice::SliceIndex;

// Sealed trait implementation
mod sealed {
    pub trait Sealed {}
    impl Sealed for i8 {}
    impl Sealed for u8 {}
}

pub trait ValueType: sealed::Sealed {}
impl ValueType for i8 {}
impl ValueType for u8 {}

// Stack-allocated version
#[derive(Debug)]
#[repr(C, align(8))]
pub struct AlignedBuffer<T, const SIZE: usize>
where
    T: ValueType,
{
    data: MaybeUninit<[T; SIZE]>,
}

// Heap-allocated version
pub struct HeapAlignedBuffer<T>
where
    T: ValueType,
{
    ptr: *mut T,
    size: usize,
}

impl<T> HeapAlignedBuffer<T>
where
    T: ValueType,
{
    /// Allocates a new buffer on the heap
    pub fn allocate(size: usize) -> Option<Self> {
        unsafe {
            let layout = core::alloc::Layout::from_size_align(
                size * mem::size_of::<T>(),
                8,
            ).ok()?;
            
            let ptr = libc::malloc(layout.size()) as *mut T;
            if ptr.is_null() {
                return None;
            }
            
            Some(Self { ptr, size })
        }
    }

    /// Returns raw pointer to the buffer
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    /// Returns raw const pointer to the buffer
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Returns slice to initialised data (unsafe)
    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe{core::slice::from_raw_parts(self.ptr, self.size)}
    }

    /// Returns mutable slice to initialised data (unsafe)
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe{core::slice::from_raw_parts_mut(self.ptr, self.size)}
    }
}

impl<T> Drop for HeapAlignedBuffer<T>
where
    T: ValueType,
{
    fn drop(&mut self) {
        unsafe {
            libc::free(self.ptr as _);
        }
    }
}

// Common implementation for stack-based buffer
impl<T, const SIZE: usize, Idx> Index<Idx> for AlignedBuffer<T, SIZE>
where
    T: ValueType,
    Idx: SliceIndex<[T]>,
{
    type Output = Idx::Output;

    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        // SAFETY: The buffer must be initialised
        unsafe { self.assume_init().get_unchecked(index) }
    }
}

impl<T, const SIZE: usize, Idx> IndexMut<Idx> for AlignedBuffer<T, SIZE>
where
    T: ValueType,
    Idx: SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        // SAFETY: The buffer must be initialised
        unsafe { self.assume_init_mut().get_unchecked_mut(index) }
    }
}

impl<T, const SIZE: usize> AlignedBuffer<T, SIZE>
where
    T: ValueType,
{
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
        }
    }

    /// Creates a heap-allocated version of this buffer
    pub fn allocate(&self) -> Option<HeapAlignedBuffer<T>> {
        HeapAlignedBuffer::allocate(SIZE)
    }

    #[inline]
    #[must_use]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr().cast()
    }

    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr().cast()
    }

    /// # Safety
    /// The buffer must be initialised before calling this
    #[inline]
    pub const unsafe fn as_slice(&self) -> &[T] {
        unsafe { &*self.data.as_ptr() }
    }

    /// # Safety
    /// The buffer must be initialised before calling this
    #[inline]
    pub const unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.data.as_mut_ptr() }
    }

    /// # Safety
    /// The range must be within initialised portion of the buffer
    #[inline]
    pub unsafe fn get_unchecked<R>(&self, range: R) -> &R::Output
    where
        R: SliceIndex<[T]>,
    {
        unsafe { self.as_slice().get_unchecked(range) }
    }

    /// # Safety
    /// The range must be within initialised portion of the buffer
    #[inline]
    pub unsafe fn get_unchecked_mut<R>(&mut self, range: R) -> &mut R::Output
    where
        R: SliceIndex<[T]>,
    {
        unsafe { self.as_mut_slice().get_unchecked_mut(range) }
    }

    /// # Safety
    /// The entire buffer must be initialised
    #[inline]
    const unsafe fn assume_init(&self) -> &[T; SIZE] {
        unsafe { &*self.data.as_ptr() }
    }

    /// # Safety
    /// The entire buffer must be initialised
    #[inline]
    const unsafe fn assume_init_mut(&mut self) -> &mut [T; SIZE] {
        unsafe { &mut *self.data.as_mut_ptr() }
    }
}

// Syscall interfaces remain the same
impl<T, const SIZE: usize> AlignedBuffer<T, SIZE>
where
    T: ValueType,
{
    /// # Safety
    /// This is only to be called when using syscalls in the getdents interface
    #[inline]
    #[cfg(target_os = "linux")]
    pub unsafe fn getdents64_internal(&mut self, fd: i32) -> i64 {
        unsafe { libc::syscall(libc::SYS_getdents64, fd, self.as_mut_ptr(), SIZE) }
    }

    /// # Safety
    /// This uses inline assembly to bypass glibc quirks
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub unsafe fn getdents64_asm(&mut self, fd: i32) -> i64 {
        use core::arch::asm;
        let output;
        unsafe {
            asm!(
                "syscall",
                inout("rax") libc::SYS_getdents64  => output,
                in("rdi") fd,
                in("rsi") self.as_mut_ptr(),
                in("rdx") SIZE,
                out("rcx") _,  // syscall clobbers rcx
                out("r11") _,  // syscall clobbers r11
                options(nostack, preserves_flags)
            )
        };
        output
    }
}

