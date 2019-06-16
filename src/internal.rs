

pub struct FrameContext {
    /*
    Dav1dRef *seq_hdr_ref;
    Dav1dSequenceHeader *seq_hdr;
    Dav1dRef *frame_hdr_ref;
    Dav1dFrameHeader *frame_hdr;
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

    const Dav1dContext *c;
    Dav1dTileContext *tc;
    int n_tc;
    Dav1dTileState *ts;
    int n_ts;
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
    int w4, h4, bw, bh, sb128w, sb128h, sbh, sb_shift, sb_step, sr_sb128w;
    uint16_t dq[DAV1D_MAX_SEGMENTS][3 /* plane */][2 /* dc/ac */];
    const uint8_t *qm[2 /* is_1d */][N_RECT_TX_SIZES][3 /* plane */];
    BlockContext *a;
    int a_sz /* w*tile_rows */;
    AV1_COMMON *libaom_cm; // FIXME
    uint8_t jnt_weights[7][7];
    int bitdepth_max;

    struct {
        struct thread_data td;
        int pass, die;
        // indexed using t->by * f->b4_stride + t->bx
        Av1Block *b;
        struct CodedBlockInfo {
            int16_t eob[3 /* plane */];
            uint8_t txtp[3 /* plane */];
        } *cbi;
        int8_t *txtp;
        // indexed using (t->by >> 1) * (f->b4_stride >> 1) + (t->bx >> 1)
        uint16_t (*pal)[3 /* plane */][8 /* idx */];
        // iterated over inside tile state
        uint8_t *pal_idx;
        coef *cf;
        // start offsets per tile
        int *tile_start_off;
    } frame_thread;

    // loopfilter
    struct {
        uint8_t (*level)[4];
        Av1Filter *mask;
        Av1Restoration *lr_mask;
        int top_pre_cdef_toggle;
        int mask_sz /* w*h */, lr_mask_sz, line_sz /* w */, lr_line_sz, re_sz /* h */;
        Av1FilterLUT lim_lut;
        int last_sharpness;
        uint8_t lvl[8 /* seg_id */][4 /* dir */][8 /* ref */][2 /* is_gmv */];
        uint8_t *tx_lpf_right_edge[2];
        pixel *cdef_line;
        pixel *cdef_line_ptr[2 /* pre, post */][3 /* plane */][2 /* y */];
        pixel *lr_lpf_line;
        pixel *lr_lpf_line_ptr[3 /* plane */];

        // in-loop filter per-frame state keeping
        int tile_row; // for carry-over at tile row edges
        pixel *p[3], *sr_p[3];
        Av1Filter *mask_ptr, *prev_mask_ptr;
    } lf;

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