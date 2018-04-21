use std::fs::File;
use std::io::{Write, Read};

const LUMA_WIDTH: usize=128;
const LUMA_HEIGHT: usize=96;
const CHROMA_WIDTH: usize=LUMA_WIDTH / 2;
const CHROMA_HEIGHT: usize=LUMA_HEIGHT / 2;

// H.264 bitstreams
static SPS: [u8; 11] = [ 0x00, 0x00, 0x00, 0x01, 0x67, 0x42, 0x00, 0x0a, 0xf8, 0x41, 0xa2 ];
static PPS: [u8; 8] = [ 0x00, 0x00, 0x00, 0x01, 0x68, 0xce, 0x38, 0x80 ];
static SLICE_HEADER: [u8; 9] = [ 0x00, 0x00, 0x00, 0x01, 0x05, 0x88, 0x84, 0x21, 0xa0 ];
static MACROBLOCK_HEADER: [u8; 2] = [ 0x0d, 0x00 ];

pub struct Frame {
   y: [[u8; LUMA_WIDTH]; LUMA_HEIGHT],
   cb: [[u8; CHROMA_WIDTH]; CHROMA_HEIGHT],
   cr: [[u8; CHROMA_WIDTH]; CHROMA_HEIGHT],
}

// Write a macroblock's worth of YUV data in I_PCM mode
pub fn macroblock(i: usize, j: usize, mut f: &File, frame: &Frame) {

  if !(i == 0 && j == 0) {
    f.write_all(&MACROBLOCK_HEADER).unwrap();
  }

  // luma / y
  for x in (i * 16)..((i + 1) * 16) {
      for y in (j * 16)..((j + 1) * 16) {
        f.write_all(&[frame.y[x][y]]).unwrap();
      }
  }

  // cb / chroma blue
  for x in (i * 8)..((i + 1) * 8) {
      for y in (j * 8)..((j + 1) * 8) {
          f.write_all(&[frame.cb[x][y]]).unwrap();
      }
  }

  // cr / chroma red
  for x in (i * 8)..((i + 1) * 8) {
      for y in (j * 8)..((j + 1) * 8) {
          f.write_all(&[frame.cr[x][y]]).unwrap();
      }
  }
}

/* Write out PPS, SPS, and loop over input, writing out I slices */
pub fn main() {

    let mut buffer = [0; 128];
    let filesize = 9216000;
    let mut bytes_read = 0;
    let mut readf = File::open("test.yuv").expect("unable to open file");

    let mut f = File::create("foo.264").expect("unable to create file");

    f.write_all(&SPS).unwrap();
    f.write_all(&PPS).unwrap();

    while bytes_read < filesize {
        let mut y = [[0; 128]; 96];
        // ok, so now we've got some bytes, let's fill frame.y
        for i in 0..96 {
            // go through every byte in the buffer
            // assign a byte to frame.y[i][j]
            buffer = [0;128];
            bytes_read += readf.read(&mut buffer).expect("couldn't read bytes for frame.y");
            y[i] = buffer;
        }

        let mut cb = [[0; CHROMA_WIDTH]; 96 / 2];
        let mut buffer = [0; CHROMA_WIDTH];
        for i in 0..(96 / 2) {
            // go through every byte in the buffer
            // assign a byte to frame.y[i][j]
            buffer = [0;128/2];
            bytes_read += readf.read(&mut buffer).expect("couldn't read bytes for frame.cb");
            cb[i] = buffer;
        }

        let mut cr = [[0; CHROMA_WIDTH]; 96 / 2];
        for i in 0..(96 / 2) {
            // go through every byte in the buffer
            // assign a byte to frame.y[i][j]
            buffer = [0;128/2];
            bytes_read += readf.read(&mut buffer).expect("couldn't read bytes for frame.cr");
            cr[i] = buffer;
        }

        let frame = &Frame{y, cb, cr};

        f.write_all(&SLICE_HEADER).unwrap();

        for i in 0..(LUMA_HEIGHT/16) {
            for j in 0..(LUMA_WIDTH/16) {
                macroblock(i, j, &f, &frame);
            }
        }
        
        // slice stop bit 
        f.write_all(&[0x80]).unwrap();
    }
}