use windows::{Win32::{Foundation::{HINSTANCE, HMODULE}, System::{Diagnostics::Debug::FlushInstructionCache, LibraryLoader::GetModuleHandleW, ProcessStatus::{GetModuleInformation, MODULEINFO}, Threading::{CreateThread, GetCurrentProcess, THREAD_CREATION_FLAGS}}}, core::w};
const DLL_PROCESS_ATTACH: u32 = 1;

mod runtime_patcher;
mod constants;

use constants::CONSTANTS;

use crate::runtime_patcher::unprotect::unlock_memory_block;

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: u32,
    lpv_reserved: *const std::ffi::c_void
) -> u32 {

    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                CreateThread(None, 
                            0, 
                            Some(ModMain),
                            None,
                            THREAD_CREATION_FLAGS(0), 
                            None).unwrap();
            }
        }

        _ => ()
    }

    // truthy
    1
}

pub unsafe extern "system" fn ModMain(
    arg: *mut std::ffi::c_void
) -> u32 {

    // go stare at constants/mod.rs and then we do 0x90 0x90 0x90 0x90 0x90 0x90 0x90 BRRRRRRRRRRRRRRRRRRRRR
    // unnescessary to add support for other os because we are mainy using windows functions
    let NOP_byte: u8 = 0x90;

    // go get gameassembly.dll what happens if we accidently unsecure the entire windows memory HAHAHAHA
    // this is impossible to panic plsplsplsplslspslsplpslsplslsplspslsplsplspslsplsplsplspslpslsplspslsplspslsplspslspl
    // wtf there is an arg in ModMain that i can use to get base address
    // let gameassembly_base = GetModuleHandleW(None).unwrap();
    unsafe {
        // #[repr(transparent)]
        // pub struct HINSTANCE(pub *mut c_void);
        // this will panic which is good if you inject it into bullshit game (try it in crysis)
        let gameassembly_base = GetModuleHandleW(w!("GameAssembly.dll")).unwrap().0 as usize;
        let mut gameassembly_size = 0;
        // grab gameassembly memory block size so that we can unprotect the whole memory of gameassembly only
        {
            // put here so that rust can drop this struct and save memory again
            let mut module_info = MODULEINFO::default();

            GetModuleInformation(
                GetCurrentProcess(),
                HMODULE(gameassembly_base as *mut std::ffi::c_void),
                &mut module_info as *mut MODULEINFO,
                std::mem::size_of::<MODULEINFO>() as u32,
            ).unwrap();

            gameassembly_size = module_info.SizeOfImage as usize;
        };

        let patch_span = CONSTANTS::to_runtime(gameassembly_base, CONSTANTS::PATCH_SPAN_START);
        unlock_memory_block(patch_span, CONSTANTS::PATCH_SPAN_LENGTH).unwrap();

        for (static_va, instruction_length) in CONSTANTS::NOP_TARGETS {
            let address = CONSTANTS::to_runtime(gameassembly_base, static_va);

            // memset, one 0x90 per byte of the original instruction
            std::ptr::write_bytes(address as *mut u8, NOP_byte, instruction_length);

            // flush to make the changes apply now in the game
            FlushInstructionCache(
                GetCurrentProcess(),
                Some(address as *const std::ffi::c_void),
                instruction_length,
            ).unwrap();
        }

        for (static_va, patch_bytes) in CONSTANTS::BYTE_PATCHES {
            let address = CONSTANTS::to_runtime(gameassembly_base, static_va);

            unlock_memory_block(address, patch_bytes.len()).unwrap();

            std::ptr::copy_nonoverlapping(patch_bytes.as_ptr(), address as *mut u8, patch_bytes.len());

            FlushInstructionCache(
                GetCurrentProcess(),
                Some(address as *const std::ffi::c_void),
                patch_bytes.len(),
            ).unwrap();
        }
    }
    1
}