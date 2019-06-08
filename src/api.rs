use crate::headers::*;
use crate::picture::*;

pub struct Rav1dUserData {
    pub data: Box<[u8]>,
    //ref: &Rav1dRef  //TODO
}

pub struct Rav1dDataProps {
    pub timestamp: u64,
    pub duration: u64,
    pub offset: i64,
    pub size: usize,
    //pub user_data: Rav1dUserData,
}

pub struct Rav1dData {
    pub data: Box<[u8]>,
    //ref: &Rav1dRef  //TODO
    pub m: Rav1dDataProps,
}

pub fn rav1d_data_create<'a>(data: &'a Rav1dData, sz: usize) -> Result<&'a [u8], isize> {
    Err(-1) // TODO: use ErrorCode, instead of isize
}

pub fn rav1d_data_wrap<'a, T>(
    data: &'a Rav1dData,
    buf: &'a [u8],
    cookie: &T,
    callback: Box<Fn(&'a [u8], &T)>,
) -> isize {
    0
}

pub fn rav1d_data_wrap_user_data<'a, T>(
    data: &'a Rav1dData,
    user_data: &'a [u8],
    cookie: &T,
    callback: Box<Fn(&'a [u8], &T)>,
) -> isize {
    0
}

pub fn rav1d_data_unref<'a>(data: &'a Rav1dData) {}

const RAV1D_MAX_FRAME_THREADS: usize = 256;
const RAV1D_MAX_TILE_THREADS: usize = 64;

/*struct Rav1dLogger<'a, T> {
    cookie: &'a T,
    callback: )(void *cookie, const char *format, va_list ap);
}*/

pub struct Rav1dSettings {
    n_frame_threads: isize,
    n_tile_threads: isize,
    apply_grain: isize,
    operating_point: isize, // select an operating point for scalable AV1 bitstreams (0 - 31)
    all_layers: isize,      // output all spatial layers of a scalable AV1 biststream
    frame_size_limit: usize, // maximum frame size, in pixels (0 = unlimited)
    reserved: [u8; 32],     // reserved for future use
                            //Rav1dPicAllocator allocator;
                            //Rav1dLogger logger;
}

pub struct Rav1dContext {

}

pub fn rav1d_version() -> String {
    String::from("")
}

pub fn rav1d_default_settings(s: &Rav1dSettings) {}

pub fn rav1d_open(s: &Rav1dSettings) -> Result<Rav1dContext, isize> {
    Err(-1)
}

pub fn rav1d_parse_sequence_header(out: &Rav1dSequenceHeader, buf: &[u8]) -> isize {
    0
}

pub fn rav1d_send_data(c: &Rav1dContext, i: &Rav1dData) -> isize {
    0
}

pub fn rav1d_get_picture(c: &Rav1dContext, out: &Rav1dPicture) -> isize {
    0
}

pub fn rav1d_close(c_out: &mut Rav1dContext) {}

pub fn rav1d_flush(c: &Rav1dContext) {}
