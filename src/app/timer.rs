use std::time::Instant;
use std::num::NonZeroU32;

const MAX_FRAME_TIMES: usize = 50;



/// #### 한국어 </br>
/// 어플리케이션에서 사용하는 타이머 입니다. </br>
/// 매번 `tick`함수를 호출하여 경과 시간과 프레임 레이트를 측정합니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the timer used by the application. </br>
/// Call the `tick` function each loop to measure the elapsed time and frame rate. </br>
/// 
#[derive(Debug, Clone, Copy)]
pub struct GameTimer {
    base_timepoint: Instant,
    previous_timepoint: Instant,
    current_time_point: Instant,

    frame_times: [f64; MAX_FRAME_TIMES],
    cnt_frame_times: usize,

    elapsed_time_sec: f64,
    fps_elapsed_time_sec: f64,
    frame_per_seconds: u64,
    frame_rate: u64,
}

impl GameTimer {
    /// #### 한국어 </br>
    /// 새로운 타이머를 생성합니다. </br>
    /// 이 함수를 호출한 시점부터 측정이 시작됩니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Create a new timer. </br>
    /// Measurement starts from the point when this function is called. </br>
    /// 
    #[inline]
    pub fn new() -> Self {
        let timepoint = Instant::now();
        Self { 
            base_timepoint: timepoint, 
            previous_timepoint: timepoint, 
            current_time_point: timepoint, 
            frame_times: [0.0; MAX_FRAME_TIMES],
            cnt_frame_times: 0,
            elapsed_time_sec: 0.0,
            fps_elapsed_time_sec: 0.0,
            frame_per_seconds: 0,
            frame_rate: 0
        }
    }

    /// #### 한국어 </br>
    /// 경과 시간과 프레임 레이트를 측정합니다. </br>
    /// `vsync`에 값이 주어질 경우 해당 프레임 레이트만큼 프로그램 실행을 지연시킵니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Measure elapsed time and frame rate. </br>
    /// If a value is given to `vsync`, program execution is delayed by the corresponding frame rate. </br>
    /// 
    pub fn tick(&mut self, vsync: Option<NonZeroU32>) {
        self.current_time_point = Instant::now();
        let mut elapsed_time_sec = self.current_time_point
            .saturating_duration_since(self.previous_timepoint)
            .as_secs_f64();

        if let Some(fps) = vsync {
            while elapsed_time_sec < (1.0 / fps.get() as f64) {
                self.current_time_point = Instant::now();
                elapsed_time_sec = self.current_time_point
                    .saturating_duration_since(self.previous_timepoint)
                    .as_secs_f64();
            }
        }

        self.previous_timepoint = self.current_time_point;

        if (self.elapsed_time_sec - elapsed_time_sec).abs() < 1.0 {
            self.frame_times.copy_within(0..(MAX_FRAME_TIMES - 1), 1);
            self.frame_times[0] = elapsed_time_sec;
            self.cnt_frame_times = (self.cnt_frame_times + 1).min(MAX_FRAME_TIMES);
        }

        self.frame_per_seconds += 1;
        self.fps_elapsed_time_sec += elapsed_time_sec;
        if self.fps_elapsed_time_sec > 1.0 {
            self.frame_rate = self.frame_per_seconds;
            self.frame_per_seconds = 0;
            self.fps_elapsed_time_sec -= 1.0;
        }

        self.elapsed_time_sec = 0.0;
        if self.cnt_frame_times > 0 {
            self.elapsed_time_sec = self.frame_times
                .iter()
                .take(self.cnt_frame_times)
                .sum();
            self.elapsed_time_sec /= self.cnt_frame_times as f64;
        }
    }

    #[inline]
    pub fn elapsed_time_sec(&self) -> f64 {
        self.elapsed_time_sec
    }

    #[inline]
    pub fn frame_rate(&self) -> u64 {
        self.frame_rate
    }

    #[inline]
    pub fn total_time_sec(&self) -> f64 {
        self.current_time_point.saturating_duration_since(
            self.base_timepoint
        ).as_secs_f64()
    }
}
