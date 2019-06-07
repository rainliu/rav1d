pub struct Rav1dUserData<'a> {
    data: &'a [u8],
    //ref: &Rav1dRef  //TODO
}

pub struct Rav1dDataProps<'a> {
    timestamp: i64,
    duration: i64,
    offset: i64,
    size: isize,
    user_data: Rav1dUserData<'a>,
}

pub struct Rav1dData<'a> {
    data: &'a [u8],
    //ref: &Rav1dRef  //TODO
    m: Rav1dDataProps<'a>,
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
