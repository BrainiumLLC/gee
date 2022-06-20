use crate::Rect;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Div {
    Px(f32),
    // X(f32),
    Ratio(f32),
    Grow(f32),
    // Pt(f32) <-- I REALLY want this
}
pub const GROW: Div = Div::Grow(1.0);
impl Div {
    pub fn px(&self) -> f32 {
        match self {
            Div::Px(px) => *px,
            // Div::X(n) => x_px * n,
            _ => 0.0, // This is the one smelly part of this API.
                      // I experimented with a separate Dim class consisting of only X and Px, but it was less ergonomic.
        }
    }

    fn px_no_grow(&self, total_px: f32) -> f32 {
        match self {
            Div::Ratio(ratio) => total_px * ratio,
            Div::Grow(_) => 0.0,
            _ => self.px(),
        }
    }

    fn px_final(&self, total_px: f32, leftover: f32, flex_total: f32) -> f32 {
        match self {
            Div::Grow(weight) => {
                if leftover > 0.0 {
                    leftover * weight / flex_total
                } else {
                    0.0
                }
            }
            _ => self.px_no_grow(total_px),
        }
    }

    fn grow(&self) -> f32 {
        match self {
            Div::Grow(weight) => *weight,
            _ => 0.0,
        }
    }
}

fn leftover(divs: impl IntoIterator<Item = Div>, total_px: f32) -> f32 {
    divs.into_iter()
        .fold(total_px, |acc, div| acc - div.px_no_grow(total_px))
}

fn flex_total(divs: impl IntoIterator<Item = Div>) -> f32 {
    divs.into_iter().fold(0.0, |acc, div| acc + div.grow())
}

fn map_to_px<const N: usize>(divs: [Div; N], total_px: f32) -> [f32; N] {
    let leftover = leftover(divs, total_px);
    let flex_total = flex_total(divs);

    divs.map(|div| div.px_final(total_px, leftover, flex_total))
}

fn map_to_px_iter(divs: impl IntoIterator<Item = Div> + Clone, total_px: f32) -> Vec<f32> {
    let leftover = leftover(divs.clone(), total_px);
    let flex_total = flex_total(divs.clone());

    divs.into_iter()
        .map(|div| div.px_final(total_px, leftover, flex_total))
        .collect()
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Rect {
    pub fn split_row<const N: usize>(&self, divs: [Div; N], align: Align) -> [Rect; N] {
        let widths = map_to_px(divs, self.width());
        let total = total(widths.iter());

        let top = self.top();
        let left = self.left();
        let bottom = self.bottom();
        let mut running_left = match align {
            Align::Start => left,
            Align::Center => left + (self.width() - total) / 2.0,
            Align::End => left + (self.width() - total),
        };

        widths.map(|width| {
            let left = running_left;
            let right = left + width;
            running_left = right;
            Rect::from_top_right_bottom_left(top, right, bottom, left)
        })
    }

    pub fn split_column<const N: usize>(&self, divs: [Div; N], align: Align) -> [Rect; N] {
        let heights = map_to_px(divs, self.height());
        let total = total(heights.iter());

        let top = self.top();
        let left = self.left();
        let right = self.right();
        let mut running_top = match align {
            Align::Start => top,
            Align::Center => top + (self.height() - total) / 2.0,
            Align::End => top + (self.height() - total),
        };

        heights.map(|height| {
            let top = running_top;
            let bottom = top + height;
            running_top = bottom;
            Rect::from_top_right_bottom_left(top, right, bottom, left)
        })
    }

    pub fn split_row_iter(
        &self,
        divs: impl IntoIterator<Item = Div> + Clone,
        align: Align,
    ) -> Vec<Rect> {
        let widths = map_to_px_iter(divs, self.width());
        let total = total(widths.iter());

        let top = self.top();
        let left = self.left();
        let bottom = self.bottom();
        let mut running_left = match align {
            Align::Start => left,
            Align::Center => left + (self.width() - total) / 2.0,
            Align::End => left + (self.width() - total),
        };

        widths
            .iter()
            .map(|width| {
                let left = running_left;
                let right = left + width;
                running_left = right;
                Rect::from_top_right_bottom_left(top, right, bottom, left)
            })
            .collect()
    }

    pub fn split_column_iter(
        &self,
        divs: impl IntoIterator<Item = Div> + Clone,
        align: Align,
    ) -> Vec<Rect> {
        let heights = map_to_px_iter(divs, self.height());
        let total = total(heights.iter());

        let top = self.top();
        let left = self.left();
        let right = self.right();
        let mut running_top = match align {
            Align::Start => top,
            Align::Center => top + (self.height() - total) / 2.0,
            Align::End => top + (self.height() - total),
        };

        heights
            .iter()
            .map(|height| {
                let top = running_top;
                let bottom = top + height;
                running_top = bottom;
                Rect::from_top_right_bottom_left(top, right, bottom, left)
            })
            .collect()
    }
}

fn total<'a>(vals: impl Iterator<Item = &'a f32>) -> f32 {
    vals.fold(0.0, |acc, val| acc + val)
}
