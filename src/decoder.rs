use super::api;

pub struct Rav1Decoder {
    pub operating_point: usize,
    pub operating_point_idc: u16,
}

pub fn rav1d_create(param: &api::Rav1DecParam) -> i32 {
    return 0;
}

pub fn rav1d_destroy(dec: &Rav1Decoder) -> i32 {
    return 0;
}

pub fn rav1d_decode(dec: &Rav1Decoder, frame: &api::Rav1DecFrame) -> i32 {
    return 0;
}

pub fn rav1d_probe(param: &api::Rav1DecParam, frame: &api::Rav1DecFrame) -> i32 {
    return 0;
}
