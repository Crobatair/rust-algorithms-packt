use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};


use crate::blob::{read_u64, write_u64, Blob};
use crate::error::BlobError;

const CONT_SIZE: u64 = 32;

pub struct BlobStore{
  file: File,
  hseed: u64,
  block_size: u64,
  nblocks: u64,
  elems: u64,
}

impl BlobStore {

  pub fn new(fname: &str, block_size: u64, nblocks: u64) -> Result<Self, BlobError> {
    let hseed = rand::random::<u64>();
    let mut ff = OpenOptions::new()
      .create_new(true)
      .write(true)
      .read(true)
      .open(fname)?;
    
    let f = &mut ff;
    f.set_len(CONT_SIZE + block_size * nblocks);
    f.seek(SeekFrom::Start(0))?;
    write_u64(f, hseed)?;
    write_u64(f, block_size)?;
    write_u64(f, nblocks)?;     
    write_u64(f, 0)?; // 0 elements on create

    // mark beginnings of each block to show empty
    for x in 0 .. nblocks {
      f.seek(SeekFrom::Start(CONT_SIZE + x * block_size))?;
      write_u64(f, 0)?;
      write_u64(f, block_size - 16)?;
    }

    Ok(
      BlobStore {
        hseed,
        file: ff,
        block_size,
        nblocks,
        elems: 0,
      }
    )
  }

  pub fn open(fname: &str) -> Result<Self, BlobError> {

    let mut ff = OpenOptions::new().write(true).read(true).open(fname)?;
    let f = &mut ff;
    f.seek(SeekFrom::Start(0))?;
    let hseed = read_u64(f)?;
    let block_size = read_u64(f)?;
    let nblocks = read_u64(f)?;
    let elems = read_u64(f)?;

    Ok(
      BlobStore {
        hseed,
        file: ff,
        block_size,
        nblocks,
        elems: elems,
      }
    )
  }

  pub fn new_or_open(fname: &str, bsize: u64, nblocks: u64) -> Result<Self, BlobError> {
    Self::new(fname, bsize, nblocks).or_else(|_| Self::open(fname))
  }


  pub fn inc_elems(&mut self, n: i32) -> Result<(), BlobError> {
    if n > 0 {
      self.elems += n as u64;
    } else {
      let n2 = (-n) as u64;
      if self.elems > n2 {
        self.elems -= n2;
      }
    }

    self.file.seek(SeekFrom::Start(24))?;
    write_u64(&mut self.file, self.elems)?;
    Ok(())
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn test_create_file() {
    let fs = "test_data/create_file";
    std::fs::remove_file(fs).ok();
    let mut bs = BlobStore::new(fs, 1000, 10).unwrap();

    let blocksize = bs.block_size;
    let mut b2 = BlobStore::open(fs).unwrap();
    assert_eq!(b2.block_size, blocksize);
  }
}