use super::api;

pub struct Rav1DecParam {
    width: isize,
    height: isize,
    handle: isize,
}

pub struct Rav1DecFrame<'a> {
    general: isize,
    bitstream: &'a [u8],
    length: isize,

    image: &'a [u8],
    stride: isize,
    colorspace: isize,
    structure: isize,
    distance: isize,
}

pub struct Rav1Decoder{

}

pub fn rav1d_create(param: &Rav1DecParam) -> i32 {
    return 0;
}

pub fn rav1d_destroy(dec: &Rav1Decoder) -> i32 {
    return 0;
}

pub fn rav1d_decode(dec: &Rav1Decoder, frame: &Rav1DecFrame) -> i32 {
    return 0;
}

pub fn rav1d_probe(param: &Rav1DecParam, frame: &Rav1DecFrame) -> i32{
    return 0;
}