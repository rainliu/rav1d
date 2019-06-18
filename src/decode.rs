use crate::api::*;
use crate::frame::Frame;
use crate::getbits::*;
use crate::headers::*;
use crate::internal::*;
use crate::levels::*;
use crate::util::*;
use crate::dequant_tables::*;

use std::rc::Rc;
use std::slice;
use std::vec::Vec;
use std::{cmp, io};

fn init_quant_tables(seq_hdr: &SequenceHeader,
                     frame_hdr: &FrameHeader,
                     qidx: i32,
                     dq: &mut AlignedArray<[[[u16;2 /* dc/ac */];3 /* plane */];MAX_SEGMENTS]>)
{
    for i in 0.. if frame_hdr.segmentation.enabled {8 }else {1} {
        let yac = if frame_hdr.segmentation.enabled {
            clip(qidx + frame_hdr.segmentation.seg_data.d[i].delta_q, 0, 255) } else {qidx};
        let ydc = clip(yac + frame_hdr.quant.ydc_delta,0,255);
        let uac = clip(yac + frame_hdr.quant.uac_delta,0,255);
        let udc = clip(yac + frame_hdr.quant.udc_delta,0,255);
        let vac = clip(yac + frame_hdr.quant.vac_delta,0,255);
        let vdc = clip(yac + frame_hdr.quant.vdc_delta,0,255);

        dq.array[i][0][0] = dq_tbl[seq_hdr.hbd as usize][ydc as usize][0];
        dq.array[i][0][1] = dq_tbl[seq_hdr.hbd as usize][yac as usize][1];
        dq.array[i][1][0] = dq_tbl[seq_hdr.hbd as usize][udc as usize][0];
        dq.array[i][1][1] = dq_tbl[seq_hdr.hbd as usize][uac as usize][1];
        dq.array[i][2][0] = dq_tbl[seq_hdr.hbd as usize][vdc as usize][0];
        dq.array[i][2][1] = dq_tbl[seq_hdr.hbd as usize][vac as usize][1];
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
            /*let refresh_frame_flags = f->frame_hdr->refresh_frame_flags;
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
        if frame_hdr.frame_is_intra() || frame_hdr.allow_intrabc {
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
        /*for (int i = f->frame_hdr->quant.qm; i < 2; i++)
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



        Ok(())
    }
}
