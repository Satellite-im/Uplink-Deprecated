pub struct Box;

#[derive(Default)]
pub struct Dimension {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct BoxDimensionsParams {
    pub container_width: f64,
    pub container_height: f64,
    pub num_boxes: i32,
    pub aspect_ratio: f64,
    pub gap: f64,
}

impl Box {
    pub fn get_box_dimensions_for_layout(
        params: BoxDimensionsParams,
        num_max_cols: i32,
        num_rows: i32,
    ) -> Dimension {
        let row_gap = params.gap * (num_rows - 1) as f64;
        let col_gap = params.gap * (num_max_cols - 1) as f64;
        let mut box_width = (params.container_width - col_gap) / num_max_cols as f64;
        let mut box_height = box_width / params.aspect_ratio;
        let content_height = box_height * num_rows as f64 + row_gap;
        if content_height > params.container_height {
            box_height = (params.container_height - row_gap) / num_rows as f64;
            box_width = box_height * params.aspect_ratio;
        }
        Dimension {
            width: box_width,
            height: box_height,
        }
    }

    pub fn get_optimal_box_dimensions(params: BoxDimensionsParams) -> Dimension {
        let mut prev_width;
        let mut prev_height;
        let mut width = 0.0;
        let mut height = 0.0;
        for num_rows in 1..params.num_boxes {
            let num_max_cols = (params.num_boxes as f64 / num_rows as f64).ceil();
            let dim = Self::get_box_dimensions_for_layout(
                params.clone(),
                num_max_cols.ceil() as i32,
                num_rows,
            );
            prev_width = dim.width;
            prev_height = dim.height;
            if prev_width > width {
                width = prev_width;
                height = prev_height;
            }
        }
        Dimension { width, height }
    }
}
