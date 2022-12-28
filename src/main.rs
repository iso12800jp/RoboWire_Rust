use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let stl_model = read_poly("./src/CylinderFlat.txt");
    let modeling_robo = read_modeling("./src/modeling.txt");

    let modeling_robo = modeling_transform(&stl_model, modeling_robo);

    stl_out(&modeling_robo, "./modeling_robo.stl", "modeling_robo");
}

/* 3次元座標を示す構造体
   w は常に1である
*/
#[derive(Copy, Clone)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Pos {
    fn new() -> Self {
        Pos {
            x: 0f64,
            y: 0f64,
            z: 0f64,
            w: 1f64,
        }
    }
}

/* ポリゴンを持つ構造体 */
struct Stl {
    pos: [Pos; 3],
    normal_vec: Pos,
}

impl Stl {
    fn new() -> Self {
        Stl {
            pos: [Pos::new(); 3],
            normal_vec: Pos::new(),
        }
    }

    

    fn cal_normal_vec(&mut self) {
        let tmp_vec = [
            Pos {
                x: self.pos[1].x - self.pos[0].x,
                y: self.pos[1].y - self.pos[0].y,
                z: self.pos[1].z - self.pos[0].z,
                w: self.pos[1].w - self.pos[0].w,
            },
            Pos {
                x: self.pos[2].x - self.pos[0].x,
                y: self.pos[2].y - self.pos[0].y,
                z: self.pos[2].z - self.pos[0].z,
                w: self.pos[2].w - self.pos[0].w,
            },
        ];
    
        let tmp_n_vec = Pos {
            x: tmp_vec[0].y * tmp_vec[1].z - tmp_vec[0].z * tmp_vec[1].y,
            y: tmp_vec[0].x * tmp_vec[1].z - tmp_vec[0].z * tmp_vec[1].x,
            z: tmp_vec[0].x * tmp_vec[1].y - tmp_vec[0].y * tmp_vec[1].x,
            w: 1f64,
        };
    
        let n_vec_len = (tmp_n_vec.x.powi(2) + tmp_n_vec.y.powi(2) + tmp_n_vec.z.powi(2)).sqrt();
    
        self.normal_vec =  Pos {
            x: tmp_n_vec.x / n_vec_len,
            y: tmp_n_vec.y / n_vec_len,
            z: tmp_n_vec.z / n_vec_len,
            w: tmp_n_vec.w,
        }
    }
}

/* ポリゴンの集合体である構造体 */
struct StlModel {
    n_stl_num: u64,
    stl: Vec<Stl>,
}

impl StlModel {
    fn new() -> Self {
        StlModel {
            n_stl_num: 0,
            stl: Vec::new(),
        }
    }
}

/* モデリングの変換行列を集めた構造体 */
struct Modeling {
    d_scale: [[f64; 4]; 4],
    d_rotate_x: [[f64; 4]; 4],
    d_rotate_y: [[f64; 4]; 4],
    d_rotate_z: [[f64; 4]; 4],
    d_shift: [[f64; 4]; 4],
    d_trans_matrix: [[f64; 4]; 4],
}

impl Modeling {
    fn new() -> Self {
        let matrix_unit = cal_matrix_unit();
        Modeling {
            d_scale: matrix_unit,
            d_rotate_x: matrix_unit,
            d_rotate_y: matrix_unit,
            d_rotate_z: matrix_unit,
            d_shift: matrix_unit,
            d_trans_matrix: matrix_unit,
        }
    }

    fn cal_d_trans_matrix(&mut self) {
        self.d_trans_matrix = self.d_shift;

        for j in 0..4 {
            self.d_trans_matrix = cal_matrix(
                &self.d_trans_matrix,
                match j {
                        0 => &self.d_rotate_z,
                        1 => &self.d_rotate_y,
                        2 => &self.d_rotate_x,
                        3 => &self.d_scale,
                        _ => panic!(),
                    }
            )
            // self.d_trans_matrix = match j {
            //     0 => &self.d_rotate_z),
            //     1 => &self.d_rotate_y),
            //     2 => &self.d_rotate_x),
            //     3 => &self.d_scale),
            //     _ => panic!(),
            // }
        }
    }
}

struct ModelingRobo {
    n_trans_num: u64,
    modeling: Vec<Modeling>,
    robo_stl_model: Vec<StlModel>,
}

impl ModelingRobo {
    fn new() -> Self {
        ModelingRobo {
            n_trans_num: 0,
            modeling: Vec::new(),
            robo_stl_model: Vec::new(),
        }
    }
}

fn read_poly(path: &str) -> StlModel {
    let mut stl_model = StlModel::new();

    let file_to_read = File::open(path).unwrap();
    let mut file_reader = BufReader::new(file_to_read);
    let mut buf = String::new();

    match file_reader.read_line(&mut buf).unwrap() {
        0 => panic!("ファイル長が間違っています."),
        _ => {
            stl_model.n_stl_num = buf
                .trim()
                .parse::<u64>()
                .expect("列をポリゴンに変換できませんでした.")
        }
    };
    buf.clear();

    for _ in 0..stl_model.n_stl_num {
        let mut stl = Stl::new();
        for j in 0..4 {
            if file_reader.read_line(&mut buf).unwrap() == 0 {
                panic!()
            }
            let tmp: Vec<f64> = buf
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            let tmp_pos = Pos {
                x: tmp[0],
                y: tmp[1],
                z: tmp[2],
                w: 1f64,
            };
            match j {
                0 => stl.normal_vec = tmp_pos,
                1 | 2 | 3 => {
                    stl.pos[j - 1] = tmp_pos;
                }
                _ => panic!(),
            }
            buf.clear();
        }
        stl_model.stl.push(stl);
    }

    stl_model
}

fn read_modeling(path: &str) -> ModelingRobo {
    let mut modeling_robo = ModelingRobo::new();

    // modeling.txt start
    let file_to_read = File::open(path).unwrap();

    let mut file_reader = BufReader::new(file_to_read);
    
    let mut buf = String::new();

    file_reader.read_line(&mut buf).unwrap();
    
    buf.clear();

    if file_reader.read_line(&mut buf).unwrap() == 0 {
        panic!();
    }

    modeling_robo.n_trans_num = buf
        .trim()
        .parse::<u64>()
        .expect("列をパーツ数に変換できませんでした.");

    buf.clear();

    for _ in 0..modeling_robo.n_trans_num {
        let mut modeling = Modeling::new();
        for j in 0..4 {
            if file_reader.read_line(&mut buf).unwrap() == 0 {
                panic!()
            }

            match j {
                0 => (),
                1 | 2 | 3 => {
                    let tmp: Vec<f64> = buf
                        .split(',')
                        .map(|s| s.trim().parse::<f64>().unwrap())
                        .collect::<Vec<f64>>();

                    match j {
                        1 => modeling.d_scale = scale(&tmp[0], &tmp[1], &tmp[2]),
                        2 => {
                            modeling.d_rotate_x = rotate_x(&tmp[0]);
                            modeling.d_rotate_y = rotate_y(&tmp[1]);
                            modeling.d_rotate_z = rotate_z(&tmp[2]);
                        }
                        3 => modeling.d_shift = shift(&tmp[0], &tmp[1], &tmp[2]),
                        _ => panic!(),
                    };
                }
                _ => panic!(),
            }
            buf.clear();
        }

        modeling_robo.modeling.push(modeling);
    }

    modeling_robo
}

fn shift(x: &f64, y: &f64, z: &f64) -> [[f64; 4]; 4] {
    let mut d_shift = cal_matrix_unit();
    d_shift[0][3] = *x;
    d_shift[1][3] = *y;
    d_shift[2][3] = *z;
    d_shift
}

fn rotate_z(z: &f64) -> [[f64; 4]; 4] {
    let mut d_rotate_z = cal_matrix_unit();
    d_rotate_z[0][0] = z.to_radians().cos();
    d_rotate_z[1][1] = d_rotate_z[0][0];
    d_rotate_z[1][0] = z.to_radians().sin();
    d_rotate_z[0][1] = d_rotate_z[1][0] * -1f64;
    d_rotate_z
}

fn rotate_y(y: &f64) -> [[f64; 4]; 4] {
    let mut d_rotate_y = cal_matrix_unit();
    d_rotate_y[0][0] = y.to_radians().cos();
    d_rotate_y[2][2] = d_rotate_y[0][0];
    d_rotate_y[0][2] = y.to_radians().sin();
    d_rotate_y[2][0] = d_rotate_y[0][2] * -1f64;
    d_rotate_y
}

fn rotate_x(x: &f64) -> [[f64; 4]; 4] {
    let mut d_rotate_x = cal_matrix_unit();
    d_rotate_x[1][1] = x.to_radians().cos();
    d_rotate_x[2][2] = d_rotate_x[1][1];
    d_rotate_x[2][1] = x.to_radians().sin();
    d_rotate_x[1][2] = d_rotate_x[2][1] * -1f64;
    d_rotate_x
}

fn scale(x: &f64, y: &f64, z: &f64) -> [[f64; 4]; 4] {
    let mut d_scale = cal_matrix_unit();
    d_scale[0][0] = *x;
    d_scale[1][1] = *y;
    d_scale[2][2] = *z;
    d_scale
}

fn cal_matrix_unit() -> [[f64; 4]; 4] {
    let mut result_matrix = [[0f64; 4]; 4];
    for i in 0..4 {
        result_matrix[i][i] = 1f64;
    }

    result_matrix
}

fn cal_matrix(matrix_a: &[[f64; 4]; 4], matrix_b: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut result_matrix = [[0f64; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result_matrix[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }

    result_matrix
}

fn cal_pos(matrix_a: &[[f64; 4]; 4], pos: &Pos) -> Pos {
    let matrix_b = [pos.x, pos.y, pos.z, pos.w];
    let mut result_matrix: [f64; 4] = [0f64; 4];
    // let mut result_matrix: [f64; 4] =  matrix_a.iter().map(|a| a.iter().zip(matrix_b.iter()).map(|(a, b)| a * b).collect::<Vec<f64>>()).collect::<Vec<Vec<f64>>>().try_into().unwrap();
    for i in 0..4 {
        for j in 0..4 {
            result_matrix[i] += matrix_a[i][j] * matrix_b[j];
        }
    }

    Pos {
        x: result_matrix[0],
        y: result_matrix[1],
        z: result_matrix[2],
        w: result_matrix[3],
    }
}

// fn cal_normal_vec(stl: &Stl) -> Pos {
//     let tmp_vec = [
//         Pos {
//             x: stl.pos[1].x - stl.pos[0].x,
//             y: stl.pos[1].y - stl.pos[0].y,
//             z: stl.pos[1].z - stl.pos[0].z,
//             w: stl.pos[1].w - stl.pos[0].w,
//         },
//         Pos {
//             x: stl.pos[2].x - stl.pos[0].x,
//             y: stl.pos[2].y - stl.pos[0].y,
//             z: stl.pos[2].z - stl.pos[0].z,
//             w: stl.pos[2].w - stl.pos[0].w,
//         },
//     ];

//     let tmp_n_vec = Pos {
//         x: tmp_vec[0].y * tmp_vec[1].z - tmp_vec[0].z * tmp_vec[1].y,
//         y: tmp_vec[0].x * tmp_vec[1].z - tmp_vec[0].z * tmp_vec[1].x,
//         z: tmp_vec[0].x * tmp_vec[1].y - tmp_vec[0].y * tmp_vec[1].x,
//         w: 1f64,
//     };

//     let n_vec_len = (tmp_n_vec.x.powi(2) + tmp_n_vec.y.powi(2) + tmp_n_vec.z.powi(2)).sqrt();

//     Pos {
//         x: tmp_n_vec.x / n_vec_len,
//         y: tmp_n_vec.y / n_vec_len,
//         z: tmp_n_vec.z / n_vec_len,
//         w: tmp_n_vec.w,
//     }
// }

fn modeling_transform(stl_model: &StlModel, mut modeling_robo: ModelingRobo) -> ModelingRobo {
    
    modeling_robo.modeling.iter_mut().for_each(
        |modeling| 
        {
            let mut robo_stl_model = StlModel {
                n_stl_num: stl_model.n_stl_num,
                stl: Vec::new(),
            };

            modeling.cal_d_trans_matrix();

            stl_model.stl.iter().for_each(
                |stl| 
                {
                    let mut converted_stl = Stl {
                        pos: [
                            cal_pos(&modeling.d_trans_matrix, &stl.pos[0]),
                            cal_pos(&modeling.d_trans_matrix, &stl.pos[1]),
                            cal_pos(&modeling.d_trans_matrix, &stl.pos[2]),
                        ],
                        normal_vec: Pos::new(),
                    };
                    converted_stl.cal_normal_vec();
    
                    robo_stl_model.stl.push(converted_stl);
                }
            );

            modeling_robo.robo_stl_model.push(robo_stl_model);
        }
    );

    modeling_robo
}

fn stl_out(modeling_robo: &ModelingRobo, path: &str, file_name: &str) {
    let file_to_write = File::create(path).unwrap();
    let mut file_writer = BufWriter::new(file_to_write);

    file_writer
        .write(format!("solid {}\n", file_name).as_bytes())
        .unwrap();
    modeling_robo.robo_stl_model.iter().for_each(|s| {
        s.stl.iter().for_each(|s| {
            file_writer
                .write(
                    format!(
                        "facet normal {} {} {}\n",
                        s.normal_vec.x, s.normal_vec.y, s.normal_vec.z
                    )
                    .as_bytes(),
                )
                .unwrap();
            file_writer
                .write(format!("outer loop\n").as_bytes())
                .unwrap();
            s.pos.iter().for_each(|p| {
                file_writer
                    .write(format!("vertex {} {} {}\n", p.x, p.y, p.z).as_bytes())
                    .unwrap();
            });
            file_writer.write(format!("endloop\n").as_bytes()).unwrap();
            file_writer.write(format!("endfacet\n").as_bytes()).unwrap();
        })
    });
    file_writer
        .write(format!("endsolid {}", file_name).as_bytes())
        .unwrap();
}
