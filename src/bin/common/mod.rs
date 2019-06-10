use rav1d::api::*;
use rav1d::frame::*;
use rav1d::util::Pixel;

use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct VideoDetails {
    pub width: usize,
    pub height: usize,
    pub bit_depth: usize,
    pub chroma_sampling: ChromaSampling,
    pub chroma_sample_position: ChromaSamplePosition,
    pub time_base: Rational,
}

impl Default for VideoDetails {
    fn default() -> Self {
        VideoDetails {
            width: 640,
            height: 480,
            bit_depth: 8,
            chroma_sampling: ChromaSampling::Cs420,
            chroma_sample_position: ChromaSamplePosition::Unknown,
            time_base: Rational { num: 30, den: 1 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FrameSummary {
    // Frame size in bytes
    pub width: usize,
    pub height: usize,
    pub pts: u64,
    pub frame_type: FrameType,
    // PSNR for Y, U, and V planes
    pub psnr: Option<(f64, f64, f64)>,
}

impl<T: Pixel> From<Frame<T>> for FrameSummary {
    fn from(frame: Frame<T>) -> Self {
        Self {
            width: frame.planes[0].cfg.width,
            height: frame.planes[0].cfg.height,
            pts: frame.pts,
            frame_type: frame.frame_type,
            psnr: frame.psnr,
        }
    }
}

impl fmt::Display for FrameSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Frame {} - {}x{} - {} bytes{}",
            self.pts,
            self.frame_type,
            self.width,
            self.height,
            if let Some(psnr) = self.psnr {
                format!(
                    " - PSNR: Y: {:.4}  Cb: {:.4}  Cr: {:.4}",
                    psnr.0, psnr.1, psnr.2
                )
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct ProgressInfo {
    // Frame rate of the video
    frame_rate: Rational,
    // The length of the whole video, in frames, if known
    total_frames: Option<usize>,
    // The time the encode was started
    time_started: Instant,
    // List of frames encoded so far
    frame_info: Vec<FrameSummary>,
    // Video size so far in bytes.
    //
    // This value will be updated in the CLI very frequently, so we cache the previous value
    // to reduce the overall complexity.
    // encoded_size: usize,
    // Whether to display PSNR statistics during and at end of encode
    show_psnr: bool,
}

impl ProgressInfo {
    pub fn new(frame_rate: Rational, total_frames: Option<usize>, show_psnr: bool) -> Self {
        Self {
            frame_rate,
            total_frames,
            time_started: Instant::now(),
            frame_info: Vec::with_capacity(total_frames.unwrap_or_default()),
            //encoded_size: 0,
            show_psnr,
        }
    }

    pub fn add_frame(&mut self, frame: FrameSummary) {
        //self.encoded_size += frame.size;
        self.frame_info.push(frame);
    }

    pub fn frames_decoded(&self) -> usize {
        self.frame_info.len()
    }

    pub fn decoding_fps(&self) -> f64 {
        let duration = Instant::now().duration_since(self.time_started);
        self.frame_info.len() as f64
            / (duration.as_secs() as f64 + duration.subsec_millis() as f64 / 1000f64)
    }

    /*pub fn video_fps(&self) -> f64 {
        self.frame_rate.num as f64 / self.frame_rate.den as f64
    }*/

    // Returns the bitrate of the frames so far, in bits/second
    /*pub fn bitrate(&self) -> usize {
        let bits = self.encoded_size * 8;
        let seconds = self.frame_info.len() as f64 / self.video_fps();
        (bits as f64 / seconds) as usize
    }*/

    // Estimates the final filesize in bytes, if the number of frames is known
    /*pub fn estimated_size(&self) -> usize {
        self.total_frames
            .map(|frames| self.encoded_size * frames / self.frames_encoded())
            .unwrap_or_default()
    }*/

    // Estimates the remaining encoding time in seconds, if the number of frames is known
    pub fn estimated_time(&self) -> f64 {
        self.total_frames
            .map(|frames| (frames - self.frames_decoded()) as f64 / self.decoding_fps())
            .unwrap_or_default()
    }

    // Number of frames of given type which appear in the video
    pub fn get_frame_type_count(&self, frame_type: FrameType) -> usize {
        self.frame_info
            .iter()
            .filter(|frame| frame.frame_type == frame_type)
            .count()
    }

    // Size in bytes of all frames of given frame type
    /*pub fn get_frame_type_size(&self, frame_type: FrameType) -> usize {
        self.frame_info.iter()
            .filter(|frame| frame.frame_type == frame_type)
            .map(|frame| frame.size)
            .sum()
    }*/

    pub fn print_summary(&self) -> String {
        let (key, inter, ionly, switch) = (
            self.get_frame_type_count(FrameType::KEY),
            self.get_frame_type_count(FrameType::INTER),
            self.get_frame_type_count(FrameType::INTRA_ONLY),
            self.get_frame_type_count(FrameType::SWITCH),
        );
        format!(
            "\
             Key: {:>6}, Inter: {:>6}, Intra_Only: {:>6}, Switch: {:>6} B\
             {}",
            key,
            inter,
            ionly,
            switch,
            if self.show_psnr {
                let psnr_y = self
                    .frame_info
                    .iter()
                    .map(|fi| fi.psnr.unwrap().0)
                    .sum::<f64>()
                    / self.frame_info.len() as f64;
                let psnr_u = self
                    .frame_info
                    .iter()
                    .map(|fi| fi.psnr.unwrap().1)
                    .sum::<f64>()
                    / self.frame_info.len() as f64;
                let psnr_v = self
                    .frame_info
                    .iter()
                    .map(|fi| fi.psnr.unwrap().2)
                    .sum::<f64>()
                    / self.frame_info.len() as f64;
                format!(
                    "\nMean PSNR: Y: {:.4}  Cb: {:.4}  Cr: {:.4}  Avg: {:.4}",
                    psnr_y,
                    psnr_u,
                    psnr_v,
                    (psnr_y + psnr_u + psnr_v) / 3.0
                )
            } else {
                String::new()
            }
        )
    }
}

impl fmt::Display for ProgressInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(total_frames) = self.total_frames {
            write!(
                f,
                "decoded {}/{} frames, {:.3} fps, est. time: {:.0} s",
                self.frames_decoded(),
                total_frames,
                self.decoding_fps(),
                self.estimated_time()
            )
        } else {
            write!(
                f,
                "decoded {} frames, {:.3} fps",
                self.frames_decoded(),
                self.decoding_fps()
            )
        }
    }
}
