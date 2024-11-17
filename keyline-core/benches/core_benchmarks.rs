#![feature(test)]

extern crate test;
use test::Bencher;
use keyline_core::prelude::*;

#[bench]
fn bench_error_creation(b: &mut Bencher) {
    b.iter(|| {
        let _err = Error::Config("test error".into());
    });
}

#[bench]
fn bench_error_conversion(b: &mut Bencher) {
    b.iter(|| {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test error");
        let _our_err: Error = io_err.into();
    });
}
