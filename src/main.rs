use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let stl_model = read_poly("./src/CylinderFlat.txt");
    let mut modeling_robo = read_modeling("./src/modeling.txt");
    modeling_transform(&stl_model, &mut modeling_robo);

    let modeling_robo = modeling_robo;

    for i in 0..modeling_robo.robo_stl_model[5].n_stl_num as usize {
        let stl = &modeling_robo.robo_stl_model[5].stl;
        // let stl = &stl_model.stl;
        for j in 0..3 {
            // print!("{:8.4}, {:8.4}, {:8.4} -> ", stl[i].pos[j].x, stl[i].pos[j].y, stl[i].pos[j].z)
            print!(
                "{:8.4}, {:8.4}, {:8.4} -> ",
                stl[i].pos[j].x, stl[i].pos[j].y, stl[i].pos[j].z
            )
        }
        println!("");
    }
}

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

struct TransParam {
    x: f64,
    y: f64,
    z: f64,
}

impl TransParam {
    fn new() -> Self {
        TransParam {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }
}

struct Stl {
    pos: [Pos; 3],
    normal_vec: Pos,
}

impl Stl {
    fn new() -> Self {
        Stl {
            pos: [Pos::new(), Pos::new(), Pos::new()],
            normal_vec: Pos::new(),
        }
    }
}

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

struct Modeling {
    d_scale: TransParam,
    d_rotate: TransParam,
    d_trans: TransParam,
    d_trans_matrix: [[f64; 4]; 4],
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
        let mut tmp_stl = Stl::new();
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
                0 => tmp_stl.normal_vec = tmp_pos,
                1 | 2 | 3 => {
                    tmp_stl.pos[j - 1] = tmp_pos;
                }
                _ => panic!(),
            }
            buf.clear();
        }
        stl_model.stl.push(tmp_stl);
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
        let mut d_scale = TransParam::new();
        let mut d_rotate = TransParam::new();
        let mut d_trans = TransParam::new();
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
                        .collect();
                    let tmp_trans_param = TransParam {
                        x: tmp[0],
                        y: tmp[1],
                        z: tmp[2],
                    };
                    match j {
                        1 => d_scale = tmp_trans_param,
                        2 => d_rotate = tmp_trans_param,
                        3 => d_trans = tmp_trans_param,
                        _ => panic!(),
                    };
                }
                _ => panic!(),
            }
            buf.clear();
        }

        modeling_robo.modeling.push(Modeling {
            d_scale,
            d_trans,
            d_rotate,
            d_trans_matrix: cal_matrix_unit(),
        });
    }

    modeling_robo
}

fn shift(x: &f64, y: &f64, z: &f64) -> [[f64; 4]; 4] {
    let mut d_trans = cal_matrix_unit();
    d_trans[0][3] = *x;
    d_trans[1][3] = *y;
    d_trans[2][3] = *z;
    d_trans
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
    let mut result_matrix = [0f64; 4];
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

fn cal_normal_vec(stl: &Stl) -> Pos {
    let tmp_vec = [
        Pos {
            x: stl.pos[1].x - stl.pos[0].x,
            y: stl.pos[1].y - stl.pos[0].y,
            z: stl.pos[1].z - stl.pos[0].z,
            w: stl.pos[1].w - stl.pos[0].w,
        },
        Pos {
            x: stl.pos[2].x - stl.pos[0].x,
            y: stl.pos[2].y - stl.pos[0].y,
            z: stl.pos[2].z - stl.pos[0].z,
            w: stl.pos[2].w - stl.pos[0].w,
        },
    ];

    let tmp_n_vec = Pos {
        x: tmp_vec[0].y * tmp_vec[1].z - tmp_vec[0].z * tmp_vec[1].y,
        y: tmp_vec[0].x * tmp_vec[1].z - tmp_vec[0].z * tmp_vec[1].x,
        z: tmp_vec[0].x * tmp_vec[1].y - tmp_vec[0].y * tmp_vec[1].x,
        w: 1f64,
    };

    let n_vec_len = (tmp_n_vec.x.powi(2) + tmp_n_vec.y.powi(2) + tmp_n_vec.z.powi(2)).sqrt();

    Pos {
        x: tmp_n_vec.x / n_vec_len,
        y: tmp_n_vec.y / n_vec_len,
        z: tmp_n_vec.z / n_vec_len,
        w: tmp_n_vec.w,
    }
}

fn modeling_transform(stl_model: &StlModel, modeling_robo: &mut ModelingRobo) {
    for i in 0..modeling_robo.n_trans_num as usize {
        modeling_robo.robo_stl_model.push(StlModel {
            n_stl_num: stl_model.n_stl_num,
            stl: Vec::new(),
        });

        // 長ったらしくて可視性が悪いので可変参照して代用
        let mut trans_model = &mut modeling_robo.modeling[i];

        trans_model.d_trans_matrix = shift(
            &trans_model.d_trans.x,
            &trans_model.d_trans.y,
            &trans_model.d_trans.z,
        );

        for j in 0..4 {
            trans_model.d_trans_matrix = match j {
                0 => cal_matrix(
                    &trans_model.d_trans_matrix,
                    &rotate_z(&trans_model.d_rotate.z),
                ),
                1 => cal_matrix(
                    &trans_model.d_trans_matrix,
                    &rotate_y(&trans_model.d_rotate.y),
                ),
                2 => cal_matrix(
                    &trans_model.d_trans_matrix,
                    &rotate_x(&trans_model.d_rotate.x),
                ),
                3 => cal_matrix(
                    &trans_model.d_trans_matrix,
                    &scale(
                        &trans_model.d_scale.x,
                        &trans_model.d_scale.y,
                        &trans_model.d_scale.z,
                    ),
                ),
                _ => panic!(),
            }
        }

        for j in 0..modeling_robo.robo_stl_model[i].n_stl_num as usize {
            modeling_robo.robo_stl_model[i].stl.push(Stl {
                pos: [
                    cal_pos(&trans_model.d_trans_matrix, &stl_model.stl[j].pos[0]),
                    cal_pos(&trans_model.d_trans_matrix, &stl_model.stl[j].pos[1]),
                    cal_pos(&trans_model.d_trans_matrix, &stl_model.stl[j].pos[2]),
                ],
                normal_vec: Pos::new(),
            });
            modeling_robo.robo_stl_model[i].stl[j].normal_vec =
                cal_normal_vec(&modeling_robo.robo_stl_model[i].stl[j]);
        }
    }
}
