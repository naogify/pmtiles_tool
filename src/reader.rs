use std::path::PathBuf;
use std::fs::File;
use std::collections::HashMap;
use memmap2::Mmap;
use serde_json;

#[derive(Debug, Eq, Hash, PartialEq)]
struct DirectoryKey {
  z: u8,
  x: u32,
  y: u32,
}

struct DirectoryEntry {
  offset: u64,
  length: u32,
}

type Directory = HashMap::<DirectoryKey, DirectoryEntry>;

pub struct Reader {
  mmap: Mmap,
  metadata_len: usize,
  pub root_entries_len: usize,
  root_dir: Directory,
  leaves: Directory,
  pub leaves_len: usize,
  leaf_level: u8,
}

fn load_directory(mmap: &Mmap, offset: usize, num_entries: usize) -> (Directory, Directory, u8) {
  let mut directory = Directory::new();
  let mut leaves = Directory::new();
  let mut leaf_level: u8 = 0;

  for i in 0..num_entries {
    let i_offset = offset + i * 17;

    let mut z_bytes = [0u8; 1];
    z_bytes.copy_from_slice(&mmap[i_offset..i_offset + 1]);
    let z = u8::from_le_bytes(z_bytes);

    let mut x_bytes = [0u8; 4];
    x_bytes[1..4].copy_from_slice(&mmap[i_offset + 1..i_offset + 4]);
    let x = u32::from_le_bytes(x_bytes);

    let mut y_bytes = [0u8; 4];
    y_bytes[1..4].copy_from_slice(&mmap[i_offset + 4..i_offset + 7]);
    let y = u32::from_le_bytes(y_bytes);

    let mut tile_off_bytes = [0u8; 8];
    tile_off_bytes[2..8].copy_from_slice(&mmap[i_offset + 7..i_offset + 13]);
    let tile_off = u64::from_le_bytes(tile_off_bytes);

    let mut tile_len_bytes = [0u8; 4];
    tile_len_bytes.copy_from_slice(&mmap[i_offset + 13..i_offset + 17]);
    let tile_len = u32::from_le_bytes(tile_len_bytes);

    // let key = DirectoryKey { z, x, y };
    let entry = DirectoryEntry { offset: tile_off, length: tile_len };
    if z & 0b10000000 > 0 {
      if leaf_level == 0 {
        leaf_level = z;
      }
      leaves.insert(DirectoryKey { z: z & 0b01111111, x, y }, entry);
    } else {
      directory.insert(DirectoryKey { z, x, y }, entry);
    }
  }

  (directory, leaves, leaf_level)
}

impl Reader {
  pub fn new(path: &PathBuf) -> Result<Reader, std::io::Error> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    assert_eq!(&0x4D50u16.to_le_bytes(), &mmap[0..2]);

    let mut root_entries_len_bytes = [0u8; 2];
    root_entries_len_bytes.copy_from_slice(&mmap[8..10]);
    let root_entries_len = u16::from_le_bytes(root_entries_len_bytes) as usize;

    let mut metadata_len_bytes = [0u8; 4];
    metadata_len_bytes.copy_from_slice(&mmap[4..8]);
    let metadata_len = u32::from_le_bytes(metadata_len_bytes) as usize;

    let (root_dir, leaves, leaf_level) = load_directory(&mmap, metadata_len, root_entries_len);
    let leaves_len = leaves.len();

    Ok(Reader {
      mmap,
      metadata_len,
      root_entries_len,
      root_dir,
      leaves,
      leaves_len,
      leaf_level,
    })
  }

  pub fn get_metadata(&self) -> serde_json::Value {
    let raw_json = &self.mmap[10..(10+self.metadata_len)];
    let json = serde_json::from_slice(raw_json).unwrap();
    json
  }

  pub fn get_version(&self) -> u16 {
    let mut version = [0u8; 2];
    version.copy_from_slice(&self.mmap[2..4]);
    u16::from_le_bytes(version)
  }

  pub fn get(&self, z: u8, x: u32, y: u32) -> Option<&[u8]> {
    if let Some(val) = self.root_dir.get(&DirectoryKey { z, x, y }) {
      let offset = val.offset;
      let length = val.length;
      let slice = &self.mmap[offset as usize..(offset + length as u64) as usize];
      return Some(slice);
    } else if self.leaves_len > 0 {
      let level_diff = z - self.leaf_level;
      let leaf = DirectoryKey { z: self.leaf_level, x: x / (1 << level_diff), y: y / (1 << level_diff) };
      if let Some(val) = self.leaves.get(&leaf) {
        let (directory, _, _) = load_directory(&self.mmap, val.offset as usize, val.length as usize / 17);
        if let Some(val) = directory.get(&DirectoryKey { z, x, y}) {
          let offset = val.offset;
          let length = val.length;
          let slice = &self.mmap[offset as usize..(offset + length as u64) as usize];
          return Some(slice);
        }
      }
    }
    None
  }

  // pub fn get_leaf_level(&self) -> u8 {
  //   let mut leaf_level = [0u8; 1];
  //   leaf_level.copy_from_slice(&self.mmap[10..11]);
  //   leaf_level[0]
  // }
}
