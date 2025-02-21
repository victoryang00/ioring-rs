use ioring_rs::{opcode, squeue, IoRing};
use slab::Slab;
use std::collections::VecDeque;
use std::net::TcpListener;
use std::{io, os::windows::prelude::RawHandle};

#[derive(Clone, Debug)]
enum Token {
    Accept,
    Poll {
        fd: RawHandle,
    },
    Read {
        fd: RawHandle,
        buf_index: usize,
    },
    Write {
        fd: RawHandle,
        buf_index: usize,
        offset: usize,
        len: usize,
    },
}
pub struct AcceptCount {
    entry: squeue::Entry,
    count: usize,
}

impl AcceptCount {
    fn new(fd: RawHandle, token: usize, count: usize) -> AcceptCount {}
}

fn main() {}
