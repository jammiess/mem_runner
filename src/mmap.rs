use core::arch::asm;
use std::ops::{Deref, DerefMut};
use std::ops::Drop;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ops::Index;

const PROT_ALL: usize = 0b111;
const MAP_PRIVATE: usize = 0x2;
const MAP_ANON: usize = 0x20;

pub struct MMAP<const N: usize> {
    pub data: *mut u8,
    _map: PhantomData<[u8; N]>,
}

impl<const N: usize> MMAP<N> {
    pub fn new() -> Result<Self, ()> {
        let address: isize;
        unsafe {
            asm!(
            "syscall",
            in("rax") 9,
            in("rdi") 0,
            in("rsi") N,
            in("rdx") PROT_ALL,
            in("r10") MAP_PRIVATE | MAP_ANON,
            in("r8") 0,
            in("r9") 0,
            lateout("rax") address,
            );
            match address {
                -1 => Err(()),
                _ => {
                    let addr = address as *mut _;
                    Ok(Self {
                        data: addr,
                        _map: PhantomData,
                    })
                }
            }
        }
    }
}

impl<const N: usize> Deref for MMAP<N> {
    type Target = [u8; N];
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.data) }
    }
}

impl<const N: usize> DerefMut for MMAP<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.data) }
    }
}

impl <const N: usize> Index<usize> for MMAP<N> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        if index > N {
            panic!("Index out of bounds");
        }
        unsafe {
            let value = self.data.add(index);
            &(*value)
        }
    }
}

impl<const N: usize> Drop for MMAP<N> {
    fn drop(&mut self) {
        unsafe {
            asm!(
            "syscall",
            in("rax") 11,
            in("rdi") self.data,
            in("rsi") N,
            lateout("rax") _,
            )
        }
    }
}
