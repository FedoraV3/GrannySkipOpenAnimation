use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::Result;
use windows::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect};

// ok this is useless we are running SINGLE THREADED so ummmmmmmmm let me just use RefCell
// nah just use atomicu32 for static because:
// pub struct PAGE_PROTECTION_FLAGS(pub u32);
// tuple struct ez to type cast safely to u32 soooooooooooooooooooo
// no just do PAGE_PROT_FLAGS.0 instead that was super unsafe
#[allow(unused)]
static OLD_PROTECTION_PERMISSIONS: AtomicU32 = AtomicU32::new(0);

// 1 success otherwise no
pub unsafe fn unlock_memory_block(base_address: usize, dwsize: usize) -> Result<()> {
    let mut old_protect  = PAGE_PROTECTION_FLAGS::default();
    unsafe {
        VirtualProtect(base_address as *const std::ffi::c_void, 
            dwsize, 
            PAGE_EXECUTE_READWRITE, 
            &mut old_protect as *mut PAGE_PROTECTION_FLAGS)?;
    }

    // deep copy rust drops old_protect after this function
    OLD_PROTECTION_PERMISSIONS.store(old_protect.0, Ordering::SeqCst);

    Ok(())
}

pub fn lock_memory_block(base_address: usize, dwsize: usize) -> Result<()> {
    let saved = OLD_PROTECTION_PERMISSIONS.swap(0, Ordering::SeqCst);
    if saved == 0 {
        anyhow::bail!("no saved protection, unlock_memory_block was never called");
    }

    let mut old_protect = PAGE_PROTECTION_FLAGS(saved);

    unsafe {
        VirtualProtect(base_address as *const std::ffi::c_void, 
                       dwsize, 
                       old_protect, 
                       &mut old_protect as *mut PAGE_PROTECTION_FLAGS)?;
    }

    Ok(())
}
