// use wasm_bindgen::prelude::*;

static ROUND_3_MATRIX: [[i32; 2]; 9] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [0, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

static CLOCKWISE_DELTAS: [[i32; 2]; 8] = [
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
];
static CLOCKWISE_INDEX_TABLE: [[usize; 3]; 3] = [[7, 0, 1], [6, 8, 2], [5, 4, 3]];

fn get_clockwise_index(dx: &i32, dy: &i32) -> usize {
    CLOCKWISE_INDEX_TABLE[(dx + 1) as usize][(dy + 1) as usize]
}

fn get_next_delta(matrix: &MatrixData, x: i32, y: i32, d_prev_x: i32, d_prev_y: i32) -> [i32; 2] {
    let prev_index = get_clockwise_index(&d_prev_x, &d_prev_y);
    let len = CLOCKWISE_DELTAS.len();
    for index in 0..len {
        let check_delta = CLOCKWISE_DELTAS[(index + prev_index + 1) % len];
        if check_value(&matrix, x + check_delta[0], y + check_delta[1]) {
            return check_delta;
        }
    }
    CLOCKWISE_DELTAS[prev_index as usize]
}

struct MatrixData {
    data: Vec<u8>,
    width: i32,
    // height: i32,
}

fn check_value(matrix: &MatrixData, x: i32, y: i32) -> bool {
    if x > matrix.width || x * y >= matrix.data.len() as i32 {
        return false;
    }
    matrix.data[(matrix.width * y + x) as usize] != 0
}

fn sum_around(matrix: &MatrixData, x: i32, y: i32) -> u8 {
    let mut sum = 0;
    for offset in CLOCKWISE_DELTAS {
        if check_value(matrix, x + offset[0], y + offset[1]) {
            sum += 1;
        }
    }
    return sum;
}

fn get_start(matrix: &MatrixData) -> [i32; 2] {
    for (i, px) in matrix.data.iter().enumerate() {
        let i = i as i32;
        if *px == 1 {
            return [i % matrix.width, i / matrix.width];
        }
    }
    panic!("trace error: all pixels are blank")
}

fn calc_bezier_control_point(
    x: i32,
    y: i32,
    prev_x: i32,
    prev_y: i32,
    next_x: i32,
    next_y: i32,
    ratio: f64,
) -> [f64; 4] {
    let delta_x = next_x - prev_x;
    let delta_y = next_y - prev_y;
    let (x, y, prev_x, prev_y, next_x, next_y) = (
        x as f64,
        y as f64,
        prev_x as f64,
        prev_y as f64,
        next_x as f64,
        next_y as f64,
    );
    let (foot_x, foot_y) = {
        if delta_x == 0 {
            // vertical
            (prev_x as f64, y as f64)
        } else if delta_y == 0 {
            // horizontal
            (x as f64, prev_y as f64)
        } else {
            let k = (next_y - prev_y) / (next_x - prev_x); // slope ratio
            let b = prev_y - k * prev_x;
            let k2 = -1.0 / k;
            let b2 = y - k2 * x;
            let foot_x = (b2 - b) / (k - k2);
            (
                foot_x, // k2x+b2=kx+b
                k * foot_x + b,
            )
        }
    };

    let mut delta_cp1x = (prev_x - foot_x) * ratio;
    let mut delta_cp1y = (prev_y - foot_y) * ratio;
    let mut delta_cp2x = (next_x - foot_x) * ratio;
    let mut delta_cp2y = (next_y - foot_y) * ratio;

    // what if they're on the same side?
    if delta_cp1x > 0.0 && delta_cp2x > 0.0 {
        if delta_cp1x < delta_cp2x {
            delta_cp1x *= -1.0;
            delta_cp1y *= -1.0;
        } else {
            delta_cp2x *= -1.0;
            delta_cp2y *= -1.0;
        }
    } else if delta_cp1x < 0.0 && delta_cp2x < 0.0 {
        if delta_cp1x > delta_cp2x {
            delta_cp1x *= -1.0;
            delta_cp1y *= -1.0;
        } else {
            delta_cp2x *= -1.0;
            delta_cp2y *= -1.0;
        }
    }
    return [
        x + delta_cp1x,
        y + delta_cp1y,
        x + delta_cp2x,
        y + delta_cp2y,
    ];
}

// TODO main fn
// #[wasm_bindgen]
// pub fn trace_path(matrix: &Vec<u8>, smooth_ratio:i32)->(){}
