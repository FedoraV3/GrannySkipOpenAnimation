pub mod CONSTANTS {
    /* this is the memory address i got from Binja, that resembles: 
        UnityEngine_Animation__Play(rcx_10, data_180c43450, 0)
        essentially, we will NOP this out (noticed there are some more assembly lets just << 4 like 3 times and then NOP Those):
        
        1801f90fd  488b154ca3a400     mov     rdx, qword [rel data_180c43450]
        1801f9104  4533c0             xor     r8d, r8d  {0x0}
        1801f9107  48895c2438         mov     qword [rsp+0x38 {__saved_rbx}], rbx
        1801f910c  e81fef4f00         call    UnityEngine_Animation__Play

        it saves more memory too by removing the parameters
    */

    // rebase the addresses -0x180000000 so they are actual offsets we can use
    pub const UNITY_ENGINE_ANIMATION_PLAY_GAMBLING_PAR_0: usize = 0x1f90fd;
    pub const UNITY_ENGINE_ANIMATION_PLAY_GAMBLING_PAR_1: usize = 0x1f9104;
    pub const UNITY_ENGINE_ANIMATION_PLAY_GAMBLING_FUNC_CALL: usize = 0x1f910c;

    pub const NOP_TARGETS: [(usize, usize); 3] = [
        (UNITY_ENGINE_ANIMATION_PLAY_GAMBLING_PAR_0, 7),    // 488b154ca3a400
        (UNITY_ENGINE_ANIMATION_PLAY_GAMBLING_PAR_1, 3),    // 4533c0
        (UNITY_ENGINE_ANIMATION_PLAY_GAMBLING_FUNC_CALL, 5) // e81fef4f00
    ];

    // 1801f90fd .. 1801f9111, the four instructions above back to back
    pub const PATCH_SPAN_START: usize = UNITY_ENGINE_ANIMATION_PLAY_GAMBLING_PAR_0;
    pub const PATCH_SPAN_LENGTH: usize = 20;

    pub const COLOR_TOY_CAPSULE_TIMER_WAIT_MOVSS: usize = 0x204c63;

    pub const BYTE_PATCHES: [(usize, &[u8]); 1] = [
        (COLOR_TOY_CAPSULE_TIMER_WAIT_MOVSS, &[0x0f, 0x57, 0xc9, 0x90, 0x90, 0x90, 0x90, 0x90])
    ];

    pub const fn to_runtime(base: usize, static_va: usize) -> usize {
        base + static_va
    }
}
