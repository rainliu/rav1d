use crate::api::*;
use crate::dequant_tables::*;
use crate::frame::Frame;
use crate::getbits::*;
use crate::headers::*;
use crate::internal::*;
use crate::levels::*;
use crate::plane::PlaneType;
use crate::util::*;

use std::rc::Rc;
use std::slice;
use std::vec::Vec;
use std::{cmp, io};

fn init_quant_tables(
    seq_hdr: &SequenceHeader,
    frame_hdr: &FrameHeader,
    qidx: i32,
    dq: &mut AlignedArray<[[[u16; 2]; 3]; MAX_SEGMENTS]>,
) {
    for i in 0..if frame_hdr.segmentation.enabled { 8 } else { 1 } {
        let yac = if frame_hdr.segmentation.enabled {
            clip(qidx + frame_hdr.segmentation.seg_data.d[i].delta_q, 0, 255)
        } else {
            qidx
        };
        let ydc = clip(yac + frame_hdr.quant.ydc_delta, 0, 255);
        let uac = clip(yac + frame_hdr.quant.uac_delta, 0, 255);
        let udc = clip(yac + frame_hdr.quant.udc_delta, 0, 255);
        let vac = clip(yac + frame_hdr.quant.vac_delta, 0, 255);
        let vdc = clip(yac + frame_hdr.quant.vdc_delta, 0, 255);

        dq.array[i][0][0] = dq_tbl[seq_hdr.hbd as usize][ydc as usize][0];
        dq.array[i][0][1] = dq_tbl[seq_hdr.hbd as usize][yac as usize][1];
        dq.array[i][1][0] = dq_tbl[seq_hdr.hbd as usize][udc as usize][0];
        dq.array[i][1][1] = dq_tbl[seq_hdr.hbd as usize][uac as usize][1];
        dq.array[i][2][0] = dq_tbl[seq_hdr.hbd as usize][vdc as usize][0];
        dq.array[i][2][1] = dq_tbl[seq_hdr.hbd as usize][vac as usize][1];
    }
}

fn reset_context(ctx: &mut BlockContext, keyframe: bool, pass: i32) {
    /*memset(ctx->intra, keyframe, sizeof(ctx->intra));
    memset(ctx->uvmode, DC_PRED, sizeof(ctx->uvmode));
    if (keyframe)
        memset(ctx->mode, DC_PRED, sizeof(ctx->mode));

    if (pass == 2) return;

    memset(ctx->partition, 0, sizeof(ctx->partition));
    memset(ctx->skip, 0, sizeof(ctx->skip));
    memset(ctx->skip_mode, 0, sizeof(ctx->skip_mode));
    memset(ctx->tx_lpf_y, 2, sizeof(ctx->tx_lpf_y));
    memset(ctx->tx_lpf_uv, 1, sizeof(ctx->tx_lpf_uv));
    memset(ctx->tx_intra, -1, sizeof(ctx->tx_intra));
    memset(ctx->tx, TX_64X64, sizeof(ctx->tx));
    if (!keyframe) {
        memset(ctx->ref, -1, sizeof(ctx->ref));
        memset(ctx->comp_type, 0, sizeof(ctx->comp_type));
        memset(ctx->mode, NEARESTMV, sizeof(ctx->mode));
    }
    memset(ctx->lcoef, 0x40, sizeof(ctx->lcoef));
    memset(ctx->ccoef, 0x40, sizeof(ctx->ccoef));
    memset(ctx->filter, DAV1D_N_SWITCHABLE_FILTERS, sizeof(ctx->filter));
    memset(ctx->seg_pred, 0, sizeof(ctx->seg_pred));
    memset(ctx->pal_sz, 0, sizeof(ctx->pal_sz));*/
}

fn setup_tile(
    seq_hdr: &SequenceHeader,
    frame_hdr: &FrameHeader,
    data: &[u8],
    tile_row: i32,
    tile_col: i32,
    tile_start_off: i32,
    sb_shift: i32,
    bw: i32,
    bh: i32,
    sr_sb128w: i32,
    sb128w: i32,
    n_tc: i32,
    ts: &mut TileState,
) {
    let col_sb_start = frame_hdr.tiling.col_start_sb[tile_col as usize] as i32;
    let col_sb128_start = col_sb_start >> (!seq_hdr.sb128) as i32;
    let col_sb_end = frame_hdr.tiling.col_start_sb[tile_col as usize + 1] as i32;
    let row_sb_start = frame_hdr.tiling.row_start_sb[tile_row as usize] as i32;
    let row_sb_end = frame_hdr.tiling.row_start_sb[tile_row as usize + 1] as i32;

    //ts.frame_thread.pal_idx = &f->frame_thread.pal_idx[tile_start_off * 2];
    //ts.frame_thread.cf = &((int32_t *) f->frame_thread.cf)[tile_start_off * 3];
    //dav1d_cdf_thread_copy(&ts.cdf, &f->in_cdf);
    ts.last_qidx = frame_hdr.quant.yac;
    ts.last_delta_lf.iter_mut().map(|x| *x = 0);

    //TODO
    //dav1d_msac_init(&ts.msac, data, sz, frame_hdr.disable_cdf_update);

    ts.tiling.row = tile_row;
    ts.tiling.col = tile_col;
    ts.tiling.col_start = col_sb_start << sb_shift;
    ts.tiling.col_end = cmp::min(col_sb_end << sb_shift, bw);
    ts.tiling.row_start = row_sb_start << sb_shift;
    ts.tiling.row_end = cmp::min(row_sb_end << sb_shift, bh);

    // Reference Restoration Unit (used for exp coding)
    let mut sb_idx;
    let mut unit_idx;
    if (frame_hdr.super_res.enabled) {
        // vertical components only
        sb_idx = (ts.tiling.row_start >> 5) * sr_sb128w;
        unit_idx = (ts.tiling.row_start & 16) >> 3;
    } else {
        sb_idx = (ts.tiling.row_start >> 5) * sb128w + col_sb128_start;
        unit_idx = ((ts.tiling.row_start & 16) >> 3) + ((ts.tiling.col_start & 16) >> 4);
    }
    for p in 0..3 {
        if frame_hdr.restoration.t[p] == RestorationType::RESTORATION_NONE {
            continue;
        }

        if frame_hdr.super_res.enabled {
            unimplemented!();
        /*const int ss_hor = p && f->cur.p.layout != DAV1D_PIXEL_LAYOUT_I444;
        const int d = frame_hdr.super_res.width_scale_denominator;
        const int unit_size_log2 = frame_hdr.restoration.unit_size[!!p];
        const int rnd = (8 << unit_size_log2) - 1, shift = unit_size_log2 + 3;
        const int x = ((4 * ts.tiling.col_start * d >> ss_hor) + rnd) >> shift;
        const int px_x = x << (unit_size_log2 + ss_hor);
        const int u_idx = unit_idx + ((px_x & 64) >> 6);
        const int sb128x = px_x >> 7;
        if (sb128x >= f->sr_sb128w) continue;
        ts.lr_ref[p] = &f->lf.lr_mask[sb_idx + sb128x].lr[p][u_idx];*/
        } else {
            //ts.lr_ref[p] = &f->lf.lr_mask[sb_idx].lr[p][unit_idx];
        }

        /*ts.lr_ref[p]->filter_v[0] = 3;
        ts.lr_ref[p]->filter_v[1] = -7;
        ts.lr_ref[p]->filter_v[2] = 15;
        ts.lr_ref[p]->filter_h[0] = 3;
        ts.lr_ref[p]->filter_h[1] = -7;
        ts.lr_ref[p]->filter_h[2] = 15;
        ts.lr_ref[p]->sgr_weights[0] = -32;
        ts.lr_ref[p]->sgr_weights[1] = 31;*/
    }

    if n_tc > 1 {
        //TODO
        //atomic_init(&ts.progress, row_sb_start);
    }
}

impl<T: Pixel> Context<T> {
    pub fn submit_frame(&mut self) -> io::Result<()> {
        // TODO:
        // initialize context
        let f_idx = if self.n_fc > 1 {
            // multi-threading
            unimplemented!()
        } else {
            0
        };

        if self.n_fc == 1 {
            // single threading
            self.decode_frame(f_idx)?;
        //if ((res = dav1d_decode_frame(f)) < 0) {
        /*let refresh_frame_flags = frame_hdr.refresh_frame_flags;
        dav1d_picture_unref_internal(&c->out);
        for (int i = 0; i < 8; i++) {
            if (refresh_frame_flags & (1 << i)) {
                if (c->refs[i].p.p.data[0])
                dav1d_thread_picture_unref(&c->refs[i].p);
                dav1d_cdf_thread_unref(&c->cdf[i]);
                dav1d_ref_dec(&c->refs[i].segmap);
                dav1d_ref_dec(&c->refs[i].refmvs);
            }
        }
        return res;*/
        //}
        } else {
            // multi-threading
            unimplemented!();
        }

        Ok(())
    }

    fn decode_frame(&mut self, f_idx: usize) -> io::Result<()> {
        let f = &mut self.fc[f_idx];
        let seq_hdr = f.seq_hdr.as_ref().unwrap();
        let frame_hdr = f.frame_hdr.as_ref().unwrap();

        if f.n_tc > 1 {
            unimplemented!();
        }

        if frame_hdr.tiling.cols * frame_hdr.tiling.rows > f.n_ts {
            //TODO: add threading-related code
            f.n_ts = frame_hdr.tiling.cols * frame_hdr.tiling.rows;
        }

        if self.n_fc > 1 {
            unimplemented!();
        }

        if f.sb128w * frame_hdr.tiling.rows > f.a.len() as i32 {
            f.a = vec![BlockContext::default(); (f.sb128w * frame_hdr.tiling.rows) as usize];
        }

        // update allocation of block contexts for above
        if f.sb128w > f.lf.line_sz {
            //TODO: add cdef_line related code
            f.lf.line_sz = f.sb128w;
        }

        //TODO: add lr-related code

        // update allocation for loopfilter masks
        if f.sb128w * f.sb128h > f.lf.mask_sz {
            //TODO: add masks related code
            f.lf.mask_sz = f.sb128w * f.sb128h;
        }

        // init ref mvs
        if !frame_hdr.frame_is_intra() || frame_hdr.allow_intrabc {
            //TODO: add ref mv related code
        }

        // setup dequant tables
        init_quant_tables(seq_hdr, frame_hdr, frame_hdr.quant.yac, &mut f.dq);
        if frame_hdr.quant.qm {
            unimplemented!();
            /*
            for j in 0..RectTxfmSize::N_RECT_TX_SIZES{
                f.qm[0][j][0] = qm_tbl[frame_hdr.quant.qm_y][0][j];
                f.qm[0][j][1] = qm_tbl[frame_hdr.quant.qm_u][1][j];
                f.qm[0][j][2] = qm_tbl[frame_hdr.quant.qm_v][1][j];
            }*/
        }
        /*for (int i = frame_hdr.quant.qm; i < 2; i++)
        for (int tx = 0; tx < N_RECT_TX_SIZES; tx++)
            for (int pl = 0; pl < 3; pl++)
                f->qm[i][tx][pl] = dav1d_qm_tbl[15][!!pl][tx];*/

        // setup jnt_comp weights
        if frame_hdr.switchable_comp_refs {
            unimplemented!();
        }

        // init loopfilter pointers
        //TODO

        // parse individual tiles per tile group
        let (mut update_set, mut tile_row, mut tile_col) = (0, 0, 0);
        let data = &self.packet.as_ref().unwrap().data;
        for i in 0..f.tile.len() {
            let mut data_offset = f.tile[i].data_offset;
            let mut size = f.tile[i].data_sz;

            for j in f.tile[i].start..=f.tile[i].end {
                let mut tile_sz;
                if j == f.tile[i].end {
                    tile_sz = size;
                } else {
                    check_error(
                        frame_hdr.tiling.n_bytes > size as u32,
                        "frame_hdr.tiling.n_bytes > size",
                    )?;
                    tile_sz = 0;
                    for k in 0..frame_hdr.tiling.n_bytes as usize {
                        tile_sz |= (data[data_offset] as usize) << (k * 8);
                        data_offset += 1;
                    }
                    tile_sz += 1;
                    size -= frame_hdr.tiling.n_bytes as usize;
                    check_error(tile_sz > size, "tile_sz > size")?;
                }

                setup_tile(
                    seq_hdr,
                    frame_hdr,
                    data,
                    //tile_sz,
                    tile_row,
                    tile_col,
                    if self.n_fc > 1 {
                        /*frame_thread.tile_start_off[j]*/
                        unimplemented!()
                    } else {
                        0
                    },
                    f.sb_shift,
                    f.bw,
                    f.bh,
                    f.sr_sb128w,
                    f.sb128w,
                    f.n_tc,
                    &mut f.ts[j as usize],
                );
                tile_col += 1;

                if tile_col == frame_hdr.tiling.cols {
                    tile_col = 0;
                    tile_row += 1;
                }
                if j == frame_hdr.tiling.update && frame_hdr.refresh_context {
                    update_set = 1;
                }
                data_offset += tile_sz;
                size -= tile_sz;
            }
        }

        // 2-pass decoding:
        // - enabled for frame-threading, so that one frame can do symbol parsing
        //   as another (or multiple) are doing reconstruction. One advantage here
        //   is that although reconstruction is limited by reference availability,
        //   symbol parsing is not. Therefore, symbol parsing can effectively use
        //   row and col tile threading, but reconstruction only col tile threading;
        // - pass 0 means no 2-pass;
        // - pass 1 means symbol parsing only;
        // - pass 2 means reconstruction and loop filtering.
        let uses_2pass = (self.n_fc > 1 && frame_hdr.refresh_context) as i32;
        f.frame_thread.pass = uses_2pass;
        while f.frame_thread.pass <= 2 * uses_2pass {
            let progress_plane_type = match f.frame_thread.pass {
                0 => PlaneType::PLANE_TYPE_ALL,
                1 => PlaneType::PLANE_TYPE_BLOCK,
                _ => PlaneType::PLANE_TYPE_Y,
            };

            for n in 0..f.sb128w * frame_hdr.tiling.rows {
                reset_context(
                    &mut f.a[n as usize],
                    frame_hdr.frame_is_intra(),
                    f.frame_thread.pass,
                );
            }

            f.frame_thread.pass += 1;
        }

        Ok(())
    }
}
