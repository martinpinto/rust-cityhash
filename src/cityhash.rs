use std::mem;

pub fn cityhash32(mut s: &[u8], len: usize) -> u32 { // mut s: &[u8]
  if len <= 24 {
  	return if len <= 12 {
        if len <= 4 {
          hash32Len0to4(s, len)
        } else {
          hash32_len_5_to_12(s, len)
        }
      } else {
        hash32_len_13_to_24(s, len)
    }
  }

  // len > 32
  let mut h = len as u32;
  let mut g = (len as u32).wrapping_mul(C1);
  let mut f = g;

  let mut a0 = rotate32(fetch32(&s[len-4..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a1 = rotate32(fetch32(&s[len-8..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a2 = rotate32(fetch32(&s[len-16..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a3 = rotate32(fetch32(&s[len-12..]).wrapping_mul(C1), 17).wrapping_mul(C2);
  let mut a4 = rotate32(fetch32(&s[len-20..]).wrapping_mul(C1), 17).wrapping_mul(C2);
   
  h ^= a0;
  h = rotate32(h, 19);
  h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
  h ^= a2;
  h = rotate32(h, 19);
  h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
  g ^= a1;
  g = rotate32(g, 19);
  g = g.wrapping_mul(5).wrapping_add(0xe6546b64);
  g ^= a3;
  g = rotate32(g, 19);
  g = g.wrapping_mul(5).wrapping_add(0xe6546b64);
  f = f.wrapping_add(a4);
  f = rotate32(f, 19).wrapping_add(113);
  f = f.wrapping_mul(5).wrapping_add(0xe6546b64);

  let mut iters = ((len - 1) / 20) as u64;
  while iters > 0 {
    let a0 = rotate32(fetch32(&s[..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a1 = fetch32(&s[4..]);
    let a2 = rotate32(fetch32(&s[8..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a3 = rotate32(fetch32(&s[12..]).wrapping_mul(C1), 17).wrapping_mul(C2);
    let a4 = fetch32(&s[16..]);
    h ^= a0;
    h = rotate32(h, 18);
    h = (h * 5).wrapping_add(0xe6546b64);
    f += a1;
    f = rotate32(f, 19);
    f = f.wrapping_mul(C1);
    g += a2;
    g = rotate32(g, 18);
    g = (g * 5).wrapping_add(0xe6546b64);
    h ^= a3 + a1;
    h = rotate32(h, 19);
    h = (h * 5).wrapping_add(0xe6546b64);
    g ^= a4;
    g = bswap32(g) * 5;
    h += a4 * 5;
    h = bswap32(h);
    f += a0;
    //#define PERMUTE3(a, b, c) do { std::swap(a, b); std::swap(a, c); } while (0)
    //PERMUTE3(f, h, g);
    mem::swap(&mut h, &mut f);

    mem::swap(&mut g, &mut f);
    s = &s[20..];
    iters -= 1;
  }

  g = rotate32(g, 11) * C1;
  g = rotate32(g, 17) * C1;
  f = rotate32(f, 11) * C1;
  f = rotate32(f, 17) * C1;
  h = rotate32(h + g, 19);
  h = h * 5 + 0xe6546b64;
  h = rotate32(h, 17) * C1;
  h = rotate32(h + f, 19);
  h = h * 5 + 0xe6546b64;
  h = rotate32(h, 17) * C1;
  return h;
}

fn hash32Len0to4(s: &[u8], len: usize) -> u32 {
  let mut b: u32 = 0;
  let mut c: u32 = 9;
  for i in 0..len {
    let v: u8 = s[i];
    b = b.wrapping_mul(C1) + v as u32;
    c ^= b;
  }
  return fmix(mur(b, mur(len as u32, c)));
}

fn fetch32(p: &[u8]) -> u32 {
    (p[0] as u32) | (p[1] as u32) << 8 | (p[2] as u32) << 16 | (p[3] as u32) << 24
}

// rotate32 is a bitwise rotate
fn rotate32(val: u32, shift: u32) -> u32 {
    if shift == 0 {
        return val;
    }
    return val >> shift | val << (32 - shift)
}

// Some primes between 2^63 and 2^64 for various uses.
pub const K0: u64 = 0xc3a5c85c97cb3127;
pub const K1: u64 = 0xb492b66fbe98f273;
pub const K2: u64 = 0x9ae16a3b2f90404f;

// Magic numbers for 32-bit hashing.  Copied from Murmur3.
pub const C1: u32 = 0xcc9e2d51;
pub const C2: u32 = 0x1b873593;

// fmix is a 32-bit to 32-bit integer hash copied from Murmur3.
fn fmix(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    return h;
}

// Mur is a helper from Murmur3 for combining two 32-bit values.
fn mur(mut a: u32, mut h: u32) -> u32 {
    a = a.wrapping_mul(C1);
    a = rotate32(a, 17);
    a = a.wrapping_mul(C2);
    h ^= a;
    h = rotate32(h, 19);
    return h.wrapping_mul(5).wrapping_add(0xe6546b64);
}

fn hash32_len_5_to_12(s: &[u8], len: usize) -> u32 {
  let mut a = len as u32;
  let mut b = len as u32 * 5;
  let mut c: u32 = 9;
  let d: u32 = b;
  a += fetch32(&s[0..]);
  b += fetch32(&s[len-4..]);
  c += fetch32(&s[((len >> 1) & 4)..]);
  return fmix(mur(c, mur(b, mur(a, d))));
}

fn hash32_len_13_to_24(s: &[u8], len: usize) -> u32 {
  let a = fetch32(&s[-4+(len>>1 as u64)..]);
  let b = fetch32(&s[4..]);
  let c = fetch32(&s[len-8..]);
  let d = fetch32(&s[len>>1..]);
  let e = fetch32(&s[0..]);
  let f = fetch32(&s[len-4..]);
  let h = len as u32;

  return fmix(mur(f, mur(e, mur(d, mur(c, mur(b, mur(a, h)))))));
}

fn bswap32(x: u32) -> u32 {
    return ((x >> 24) & 0xFF) | ((x >> 8) & 0xFF00) |
        ((x << 8) & 0xFF0000) | ((x << 24) & 0xFF000000)
}
