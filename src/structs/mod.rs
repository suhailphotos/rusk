//! Minimal, runnable examples for structs.
//! ```rust
//! #[derive(Debug)]
//! struct R { w: u32, h: u32 }
//! let r = R { w: 2, h: 3 };
//! assert_eq!(r.w * r.h, 6);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn area(&self) -> u32 { self.w * self.h }
    pub fn make_square(&mut self, s: u32) { self.w = s; self.h = s; }
}

pub fn area_ref(r: &Rect) -> u32 { r.w * r.h }

pub fn demo() {
    let mut r = Rect { w: 30, h: 50 };
    println!("area (method): {}", r.area());
    println!("area (free): {}", area_ref(&r));
    println!("rect (debug): {:?}", r);
    r.make_square(20);
    println!("made square: {:?}, area={}", r, r.area());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn area_works() {
        let r = Rect { w: 3, h: 5 };
        assert_eq!(r.area(), 15);
        assert_eq!(area_ref(&r), 15);
    }
    #[test] fn make_square_works() {
        let mut r = Rect { w: 2, h: 5 };
        r.make_square(7);
        assert_eq!(r, Rect { w: 7, h: 7 });
    }
}
