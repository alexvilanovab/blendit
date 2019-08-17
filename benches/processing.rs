#![feature(test)]

extern crate test;

use blendit::*;
use test::Bencher;

#[bench]
fn processing_speed(b: &mut Bencher) {
    b.iter(|| {
        let txt = "Hello World!";
        let font = Font::new(24);
        let img = ImageBuffer::new(50, 50);

        process(img, txt, &font, || {});
    });
}
