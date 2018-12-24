use super::decoder;

pub enum Rav1Dec {
    Rav1DecDecode = 0,
    Rav1DecCreate,
    Rav1DecDestroy,
    Rav1DecProbe,
}

pub enum Rav1Err {
    Rav1ErrFail = -1,
    Rav1ErrOK = 0,
    Rav1ErrFormat = 1,
    Rav1ErrMemory = 2,
    Rav1ErrBitstream = 3,
}

pub struct Rav1InitHandle {
    // Reserved for future use
}

pub struct Rav1InitParam {
    cpu_flags: isize,
    api_version: isize,
    core_build: isize,
}

pub struct Rav1InitParamEx {
    // Reserved for future use
}

pub fn rav1_init(
    handle: &Rav1InitHandle,
    opt: isize,
    param1: &Rav1InitParam,
    param2: &Rav1InitParamEx,
) -> Rav1Err {
    return Rav1Err::Rav1ErrOK;
}

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

pub enum Rav1DecoreParam<'a> {
    Param(&'a Rav1DecParam),
    Frame(&'a Rav1DecFrame<'a>),
}

pub enum Rav1DecoreParamEx {
    // Reserved for future use
}

pub fn rav1_decore<'a>(
    handle: Option<&decoder::Rav1Decoder>,
    opt: isize,
    param1: Rav1DecoreParam,
    param2: Rav1DecoreParamEx,
) -> Rav1Err {
    return Rav1Err::Rav1ErrOK;
}
