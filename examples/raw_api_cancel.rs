use ioring_rs::windows::{
    win_ring, win_ring_cqe_iter, win_ring_get_sqe, win_ring_prep_read,
    win_ring_prep_register_buffers, win_ring_prep_register_files, win_ring_queue_exit,
    win_ring_queue_init, win_ring_sqe_set_data64, win_ring_submit_and_wait, IORING_BUFFER_INFO,
    IORING_REGISTERED_BUFFER, _NT_IORING_BUFFERREF, _NT_IORING_HANDLEREF,
    _NT_IORING_OP_FLAGS_NT_IORING_OP_FLAG_NONE,
    _NT_IORING_OP_FLAGS_NT_IORING_OP_FLAG_REGISTERED_BUFFER,
    _NT_IORING_OP_FLAGS_NT_IORING_OP_FLAG_REGISTERED_FILE,
    _NT_IORING_REG_FILES_REQ_FLAGS_NT_IORING_REG_FILES_REQ_FLAG_NONE,
};
use std::{fs, io, io::prelude::*, thread};

unsafe fn clear_cqes(ring: *mut win_ring, string: &str) -> io::Result<()> {
    win_ring_submit_and_wait(ring, u32::MAX);
    for i in (*(*ring).info.__bindgen_anon_2.CompletionQueue).Head
        ..(*(*ring).info.__bindgen_anon_2.CompletionQueue).Tail
    {
        dbg!(i);
        let cqe = win_ring_cqe_iter(ring, i);
        dbg!(
            (*cqe).__bindgen_anon_1.ResultCode,
            (*cqe).Information,
            (*cqe).UserData,
            string
        );
    }
    Ok(())
}
fn main() -> std::io::Result<()> {
    unsafe {
        let (mut hReadPipe, mut hWritePipe) = os_pipe::pipe()?;

        Ok(())
    }
}
