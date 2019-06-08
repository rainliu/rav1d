use crate::api::*;
use crate::headers::*;

/* Number of bytes to align AND pad picture memory buffers by, so that SIMD
 * implementations can over-read by a few bytes, and use aligned read/write
 * instructions. */
pub const RAV1D_PICTURE_ALIGNMENT: usize = 32;

pub struct Rav1dPictureParameters {
    w: isize,                 // width (in pixels)
    h: isize,                 // height (in pixels)
    layout: Rav1dPixelLayout, // format of the picture
    bpc: isize,               // bits per pixel component (8 or 10)
}

pub struct Rav1dPicture {
    seq_hdr: Rav1dSequenceHeader,
    frame_hdr: Rav1dFrameHeader,

    data: [Box<[u8]>; 3],

    stride: [usize; 2],

    p: Rav1dPictureParameters,
    m: Rav1dDataProps,
    /*
       content_light: &'a Rav1dContentLightLevel,

       mastering_display: Rav1dMasteringDisplay,

       frame_hdr_ref: &'a Rav1dRef<'a, T>,
       seq_hdr_ref: &'a Rav1dRef<'a, T>, // Frame parameter allocation origins
       content_light_ref: &'a Rav1dRef<'a, T>,
       mastering_display_ref: &'a Rav1dRef<'a, T>, // Metadata allocation origins
       ref_frame: &'a Rav1dRef<'a, T>,             // Frame data allocation origin

       allocator_data: &'a T, // pointer managed by the allocator
    */
}
/*
pub struct Rav1dPicAllocator<'a, T> {
    cookie: &'a T, // custom data to pass to the allocator callbacks.
    alloc_picture_callback: Box<Fn(&'a Rav1dPicture<'a, T>, &'a T) -> isize>,

    release_picture_callback: Box<Fn(&'a Rav1dPicture<'a, T>, &'a T)>,
}

pub fn dav1d_picture_unref<'a, T>(p: &'a Rav1dPicture<'a, T>) {}
*/
