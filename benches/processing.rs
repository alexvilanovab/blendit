#![feature(test)]

extern crate test;

use blendit::*;
use test::Bencher;

#[bench]
fn processing_speed(b: &mut Bencher) {
    b.iter(|| {
        let txt = "Hello World!";
        let font = Font::new(24);
        let img = ImageBuffer::new(20, 20);

        process(img, txt, &font, || {});
    });
}
