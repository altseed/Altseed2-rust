// Original Code License:
// --Easing Functions(Equations)--

// MIT License

// Copyright Â© 2001 Robert Penner

// Permission is hereby granted,
// free of charge, to any person obtaining a copy of this software and associated documentation files(the "Software"),
// to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and / or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions :

// The above copyright notice and this permission notice shall be included in all copies
// or
// substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS",
// WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::f32::consts::PI;

/// ???????????
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Easing {
    Linear,
    InSine,
    OutSine,
    InOutSine,
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InExpo,
    OutExpo,
    InOutExpo,
    InCirc,
    OutCirc,
    InOutCirc,
    InBack,
    OutBack,
    InOutBack,
    InElastic,
    OutElastic,
    InOutElastic,
    InBounce,
    OutBounce,
    InOutBounce,
}

impl Easing {
    /// ????????????????
    /// # Arguments
    /// * `t` - 0.0 ~ 1.0
    pub fn calculate(&self, mut t: f32) -> f32 {
        if t <= 0.0 {
            return 0.0;
        }
        if t >= 1.0 {
            return 1.0;
        }

        match *self {
            Easing::Linear => t,
            Easing::InSine => 1.0 - (t * PI * 0.5).cos(),
            Easing::OutSine => (t * PI * 0.5).sin(),
            Easing::InOutSine => -0.5 * ((t * PI).cos() - 1.0),
            Easing::InQuad => t * t,
            Easing::OutQuad => t * (2.0 - t),
            Easing::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    t * (4.0 - 2.0 * t) - 1.0
                }
            }
            Easing::InCubic => t * t * t,
            Easing::OutCubic => {
                t -= 1.0;
                t * t * t + 1.0
            }
            Easing::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    t -= 1.0;
                    1.0 + t * (2.0 * t) * (2.0 * t)
                }
            }
            Easing::InQuart => t * t * t * t,
            Easing::OutQuart => {
                t -= 1.0;
                -(t * t * t * t - 1.0)
            }
            Easing::InOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    t = (t - 1.0) * (t - 1.0);
                    1.0 - 8.0 * t * t
                }
            }

            Easing::InQuint => t * t * t * t * t,

            Easing::OutQuint => {
                let t_ = (t - 1.0) * (t - 1.0);
                (t - 1.0) * t_ * t_ + 1.0
            }

            Easing::InOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let t_ = (t - 1.0) * (t - 1.0);
                    1.0 + 16.0 * (t - 1.0) * t_ * t_
                }
            }

            Easing::InExpo => 2.0f32.powf(10.0 * (t - 1.0)),
            Easing::OutExpo => 2.0f32.powf(-10.0 * t) + 1.0,
            Easing::InOutExpo => {
                if t < 0.5 {
                    (2.0f32.powf(16.0 * t) - 1.0) / 510.0
                } else {
                    1.0 - 0.5 * 2.0f32.powf(-16.0 * (t - 0.5))
                }
            }

            Easing::InCirc => 1.0 - (1.0 - t).sqrt(),

            Easing::OutCirc => t.sqrt(),

            Easing::InOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - 2.0 * t).sqrt()) * 0.5
                } else {
                    (1.0 + (2.0 * t - 1.0).sqrt()) * 0.5
                }
            }

            Easing::InBack => t * t * (2.70158 * t - 1.70158),

            Easing::OutBack => {
                t = t - 1.0;
                1.0 + t * t * (2.70158 * t + 1.70158)
            }

            Easing::InOutBack => {
                if t < 0.5 {
                    t * t * (7.0 * t - 2.5) * 2.0
                } else {
                    t -= 1.0;
                    1.0 + t * t * 2.0 * (7.0 * t + 2.5)
                }
            }

            Easing::InElastic => t * t * t * t * (t * PI * 4.5).sin(),

            Easing::OutElastic => {
                let t_ = (t - 1.0) * (t - 1.0);
                1.0 - t_ * t_ * (t * PI * 4.5).cos()
            }

            Easing::InOutElastic => {
                if t < 0.45 {
                    return 8.0 * t * t * t * t * (t * PI * 9.0).sin();
                }
                if t < 0.55 {
                    return 0.5 + 0.75 * (t * PI * 4.0).sin();
                }

                let t_ = (t - 1.0) * (t - 1.0);
                1.0 - 8.0 * t_ * t_ * (t * PI * 9.0).sin()
            }

            Easing::InBounce => 2.0f32.powf(6.0 * (t - 1.0)) * (t * PI * 3.5).sin().abs(),

            Easing::OutBounce => 1.0 - 2.0f32.powf(-6.0 * t) * (t * PI * 3.5).cos().abs(),

            Easing::InOutBounce => {
                if t < 0.5 {
                    8.0 * 2.0f32.powf(8.0 * (t - 1.0)) * (t * PI * 7.0).sin().abs()
                } else {
                    1.0 - 8.0 * 2.0f32.powf(-8.0 * t) * (t * PI * 7.0).sin().abs()
                }
            }
        }
    }
}
