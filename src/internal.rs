use std::rc::Rc;

use crate::headers::*;
use crate::levels::*;
use crate::util::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct TileGroup {
    pub data_offset: usize,
    pub data_sz: usize,
    pub start: i32,
    pub end: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct loopfilter {
    /*uint8_t (*level)[4];
    Av1Filter *mask;
    Av1Restoration *lr_mask;
    int top_pre_cdef_toggle;*/
    pub(crate) mask_sz: i32, /* w*h */
    pub(crate) lr_mask_sz: i32,
    pub(crate) line_sz: i32, /* w */
    pub(crate) lr_line_sz: i32,
    pub(crate) re_sz: i32, /* h */
    //Av1FilterLUT lim_lut;
    pub(crate) last_sharpness: i32,
    /*uint8_t lvl[8 /* seg_id */][4 /* dir */][8 /* ref */][2 /* is_gmv */];
    uint8_t *tx_lpf_right_edge[2];
    pixel *cdef_line;
    pixel *cdef_line_ptr[2 /* pre, post */][3 /* plane */][2 /* y */];
    pixel *lr_lpf_line;
    pixel *lr_lpf_line_ptr[3 /* plane */];

    // in-loop filter per-frame state keeping
    int tile_row; // for carry-over at tile row edges
    pixel *p[3], *sr_p[3];
    Av1Filter *mask_ptr, *prev_mask_ptr;*/
}

#[derive(Clone)]
#[repr(C)]
pub struct FrameContext {
    pub(crate) seq_hdr: Option<Rc<SequenceHeader>>,
    pub(crate) frame_hdr: Option<Rc<FrameHeader>>,
    /*
    Dav1dThreadPicture refp[7];
    Dav1dPicture cur; // during block coding / reconstruction
    Dav1dThreadPicture sr_cur; // after super-resolution upscaling
    Dav1dRef *mvs_ref;
    refmvs *mvs, *ref_mvs[7];
    Dav1dRef *ref_mvs_ref[7];
    Dav1dRef *cur_segmap_ref, *prev_segmap_ref;
    uint8_t *cur_segmap;
    const uint8_t *prev_segmap;
    unsigned refpoc[7], refrefpoc[7][7];
    uint8_t gmv_warp_allowed[7];
    CdfThreadContext in_cdf, out_cdf;
    struct Dav1dTileGroup *tile;
    int n_tile_data_alloc;
    int n_tile_data;

    // for scalable references
    struct ScalableMotionParams {
        int scale; // if no scaling, this is 0
        int step;
    } svc[7][2 /* x, y */];
    int resize_step[2 /* y, uv */], resize_start[2 /* y, uv */];

    const Dav1dContext *c;*/
    pub(crate) tc: Vec<TileContext>,
    pub(crate) n_tc: i32,
    pub(crate) ts: Vec<TileState>,
    pub(crate) n_ts: i32,
    /*
    const Dav1dDSPContext *dsp;
    struct {
        recon_b_intra_fn recon_b_intra;
        recon_b_inter_fn recon_b_inter;
        filter_sbrow_fn filter_sbrow;
        backup_ipred_edge_fn backup_ipred_edge;
        read_coef_blocks_fn read_coef_blocks;
    } bd_fn;

    int ipred_edge_sz;
    pixel *ipred_edge[3];
    ptrdiff_t b4_stride;
    */
    pub(crate) w4: i32,
    pub(crate) h4: i32,
    pub(crate) bw: i32,
    pub(crate) bh: i32,
    pub(crate) sb128w: i32,
    pub(crate) sb128h: i32,
    pub(crate) sbh: i32,
    pub(crate) sb_shift: i32,
    pub(crate) sb_step: i32,
    pub(crate) sr_sb128w: i32,
    pub(crate) dq: AlignedArray<[[[u16;2 /* dc/ac */];3 /* plane */];MAX_SEGMENTS]>,
    //pub(crate) qm: [[[u8;2 /* is_1d */]; RectTxfmSize::N_RECT_TX_SIZES as usize];3 /* plane */],
    pub(crate) a: Vec<BlockContext>,
    pub(crate) lf: loopfilter,
    /*
    // threading (refer to tc[] for per-thread things)
    struct FrameTileThreadData {
        uint64_t available;
        pthread_mutex_t lock;
        pthread_cond_t cond, icond;
        int tasks_left, num_tasks;
        int (*task_idx_to_sby_and_tile_idx)[2];
        int titsati_sz, titsati_init[3];
        int inited;
    } tile_thread;*/
}

impl Default for FrameContext {
    fn default() -> Self {
        FrameContext {
            seq_hdr: None,
            frame_hdr: None,

            tc: vec![],
            n_tc: 0,
            ts: vec![],
            n_ts: 0,
            w4: 0,
            h4: 0,
            bw: 0,
            bh: 0,
            sb128w: 0,
            sb128h: 0,
            sbh: 0,
            sb_shift: 0,
            sb_step: 0,
            sr_sb128w: 0,
            dq:UninitializedAlignedArray(),
            a: vec![],
            lf: loopfilter::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
struct tiling {
    // in 4px units
    col_start: i32,
    col_end: i32,
    row_start: i32,
    row_end: i32,

    // in tile units
    col: i32,
    row: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct TileState {
    tiling: tiling,
    /*CdfContext cdf;
    MsacContext msac;

    atomic_int progress; // in sby units, TILE_ERROR after a decoding error
    struct {
        pthread_mutex_t lock;
        pthread_cond_t cond;
    } tile_thread;
    struct {
        uint8_t *pal_idx;
        coef *cf;
    } frame_thread;

    uint16_t dqmem[DAV1D_MAX_SEGMENTS][3 /* plane */][2 /* dc/ac */];
    const uint16_t (*dq)[3][2];
    int last_qidx;

    int8_t last_delta_lf[4];
    uint8_t lflvlmem[8 /* seg_id */][4 /* dir */][8 /* ref */][2 /* is_gmv */];
    const uint8_t (*lflvl)[4][8][2];

    Av1RestorationUnit *lr_ref[3];*/
}

#[derive(Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub struct TileContext {
    /*const Dav1dFrameContext *f;
Dav1dTileState *ts;
int bx, by;
BlockContext l, *a;
coef *cf;
pixel *emu_edge; // stride=192 for non-SVC, or 320 for SVC
// FIXME types can be changed to pixel (and dynamically allocated)
// which would make copy/assign operations slightly faster?
uint16_t al_pal[2 /* a/l */][32 /* bx/y4 */][3 /* plane */][8 /* palette_idx */];
ALIGN(uint16_t pal[3 /* plane */][8 /* palette_idx */], 16);
uint8_t pal_sz_uv[2 /* a/l */][32 /* bx4/by4 */];
uint8_t txtp_map[32 * 32]; // inter-only
Dav1dWarpedMotionParams warpmv;
union {
void *mem;
uint8_t *pal_idx;
int16_t *ac;
pixel *interintra, *lap;
int16_t *compinter;
} scratch;
ALIGN(uint8_t scratch_seg_mask[128 * 128], 32);

Av1Filter *lf_mask;
int8_t *cur_sb_cdef_idx_ptr;
// for chroma sub8x8, we need to know the filter for all 4 subblocks in
// a 4x4 area, but the top/left one can go out of cache already, so this
// keeps it accessible
enum Filter2d tl_4x4_filter;

struct {
struct thread_data td;
struct FrameTileThreadData *fttd;
int die;
} tile_thread;*/}

#[derive(Clone, Default)]
#[repr(C)]
pub struct BlockContext {
    mode: AlignedArray<[u8; 32]>,
    lcoef: AlignedArray<[u8; 32]>,
    ccoef: AlignedArray<[[u8; 32]; 2]>,
    seg_pred: AlignedArray<[u8; 32]>,
    skip: AlignedArray<[u8; 32]>,
    skip_mode: AlignedArray<[u8; 32]>,
    intra: AlignedArray<[u8; 32]>,
    comp_type: AlignedArray<[u8; 32]>,
    ref_frame: AlignedArray<[[i8; 32]; 2]>, // -1 means intra
    filter: AlignedArray<[[u8; 32]; 2]>,    // 3 means unset
    tx_intra: AlignedArray<[i8; 32]>,
    tx: AlignedArray<[i8; 32]>,
    tx_lpf_y: AlignedArray<[u8; 32]>,
    tx_lpf_uv: AlignedArray<[u8; 32]>,
    partition: AlignedArray<[u8; 16]>,
    uvmode: AlignedArray<[u8; 32]>,
    pal_sz: AlignedArray<[u8; 32]>,
}
