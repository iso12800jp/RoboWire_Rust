use std::{fs::File, io::{BufReader, BufRead}, f64::consts, str};

fn main () {

    let (stl_model, mut modeling_robo) = 
        read_data("./src/CylinderFlat.txt", "./src/modeling.txt");

    modeling_transform(&stl_model, &mut modeling_robo);
    // modeling_transform(&mut wire_model, &mut modeling_robo);

    // for i in 0..modeling_robo.robo_wire_model[5].n_wire_num as usize {
    //     for j in 0..4 as usize {
    //         print!("{}, ", modeling_robo.robo_wire_model[5].wire[i].begin.x);
    //         print!("{}, ", modeling_robo.robo_wire_model[5].wire[i].begin.y);
    //         print!("{}, ", modeling_robo.robo_wire_model[5].wire[i].begin.z);
    //     }
    //     print!(" -> ");
    //     for j in 0..4 as usize {
    //         print!("{}, ", modeling_robo.robo_wire_model[5].wire[i].begin.x);
    //         print!("{}, ", modeling_robo.robo_wire_model[5].wire[i].begin.y);
    //         print!("{}, ", modeling_robo.robo_wire_model[5].wire[i].begin.z);
    //     }
    //     print!("\n")
    // }

}

struct  Pos {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Pos {
    fn new() -> Self {
        Pos { x: 0f64, y: 0f64, z: 0f64, w: 0f64 }
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
            normal_vec: Pos::new()
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
    d_scale: [f64; 3],
    d_rotate: [f64; 3],
    d_trans: [f64; 3],
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

fn read_data (poly_path: &str, modeling_path: &str) -> (StlModel, ModelingRobo) {
    (
        read_poly(poly_path),
        read_modeling(modeling_path)
    )
}

fn read_poly(path: &str) -> StlModel {
    let mut stl_model = StlModel::new();

    let file_to_read = File::open(path).unwrap();
    let mut file_reader = BufReader::new(file_to_read);
    let mut buf = String::new();

    file_reader.read_line(&mut buf).unwrap();
    buf.clear();

    match file_reader.read_line(&mut buf).unwrap() {
        0 => panic!("ファイル長が間違っています."),
        _ => {
            stl_model.n_stl_num = buf.trim().parse().expect("列をポリゴンに変換できませんでした.")
        },
    };
    buf.clear();

    while file_reader.read_line(&mut buf).unwrap() != 0 {
        // let buf_repcale_space = buf.trim().replace(" ", ",");
        let mut tmp : Vec<Vec<&str>> = Vec::new();
        for _ in 0..4 {
            tmp.push(buf.split(',').collect());
        }
        let mut tmp_f64: Vec<Vec<f64>> = vec![vec![0f64; tmp[0].len()]; tmp.len()];

        // &stlからf64に変換したベクタをコピー
        for i in 0..tmp.len() {
            for j in 0..tmp[0].len() {
                tmp_f64[i][j] = tmp[i][j].parse::<f64>().unwrap();
            }
        }
        stl_model.stl.push(
            Stl {
                pos: [
                    Pos { x: tmp_f64[1][0], y: tmp_f64[1][1], z: tmp_f64[1][2], w: tmp_f64[1][3] },
                    Pos { x: tmp_f64[2][0], y: tmp_f64[2][1], z: tmp_f64[2][2], w: tmp_f64[2][3] },
                    Pos { x: tmp_f64[3][0], y: tmp_f64[3][1], z: tmp_f64[3][2], w: tmp_f64[3][3] },
                ],
                normal_vec: Pos { x: tmp_f64[0][0], y: tmp_f64[0][1], z: tmp_f64[0][2], w: tmp_f64[0][3] }
            }
        );
        buf.clear();
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

    match file_reader.read_line(&mut buf).unwrap() {
        0 => panic!("ファイル長が間違っています."),
        _ => {
            modeling_robo.n_trans_num = buf.trim().parse().expect("列をパーツ数に変換できませんでした.");
        },
    };
    buf.clear();

    for _i in 0..modeling_robo.n_trans_num {
        let mut d_scale = [0f64; 3];
        let mut d_rotate = [0f64; 3];
        let mut d_trans = [0f64; 3];
        for j in 0..4 {
            match file_reader.read_line(&mut buf).unwrap() {
                0 => panic!("データ点数が不正です."),
                _ => (),
            }
            
            let tmp: Vec<&str> = buf.trim().split(',').collect();
                    
            match j {
                0 => (),
                1 => d_scale = [tmp[0].parse().unwrap(), tmp[1].parse().unwrap(), tmp[2].parse().unwrap()],
                2 => d_rotate = [tmp[0].parse().unwrap(), tmp[1].parse().unwrap(), tmp[2].parse().unwrap()],
                3 => {
                    d_trans = [tmp[0].parse().unwrap(), tmp[1].parse().unwrap(), tmp[2].parse().unwrap()];
                },
                _ => panic!()
            }
            buf.clear();
        }
        modeling_robo.modeling.push(
            Modeling { 
                d_scale,
                d_rotate,
                d_trans,
                d_trans_matrix: [[0f64; 4]; 4]
            }
        );
    }

    modeling_robo
}

fn cal_matrix_unit() -> [[f64; 4]; 4]{
    let mut result_matrix = [[0f64; 4]; 4];
    for i in 0..4 {
        result_matrix[i][i] = 1f64;
    }
    
    return result_matrix;
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

    return result_matrix;
}

fn cal_pos(matrix_a: &[[f64; 4]; 4], pos: &Pos) -> Pos {
    let matrix_b = [pos.x, pos.y, pos.z, pos.w];
    let mut result_matrix = [0f64; 4];
    for i in 0..4 {
        for j in 0..4 {
            result_matrix[i] += matrix_a[i][j] * matrix_b[j];
        }
    }
    
    Pos { x: result_matrix[0], y: result_matrix[1], z: result_matrix[2], w: result_matrix[3] }
}

fn cal_normal_vec(stl: &Stl) -> Pos {
    let mut normal_vec= Pos::new();
    let tmp_n_vec = [
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
        }
    ];
    
    
    Pos::new()
}

fn modeling_transform(stl_model: &StlModel, modeling_robo: &mut ModelingRobo) {
    for i in 0..modeling_robo.n_trans_num as usize {
        modeling_robo.robo_stl_model.push(
            StlModel {
                n_stl_num: stl_model.n_stl_num,
                stl: Vec::new(),
            }
        );
        
        modeling_robo.modeling[i].d_trans_matrix = cal_matrix_unit();
        modeling_robo.modeling[i].d_trans_matrix[0][3] = modeling_robo.modeling[i].d_trans[0];
        modeling_robo.modeling[i].d_trans_matrix[1][3] = modeling_robo.modeling[i].d_trans[1];
        modeling_robo.modeling[i].d_trans_matrix[2][3] = modeling_robo.modeling[i].d_trans[2];

        for j in 0..4 {
            modeling_robo.modeling[i].d_trans_matrix = match j {
                0 => {
                    let mut matrix_b = cal_matrix_unit();
                    matrix_b[0][0] = (modeling_robo.modeling[i].d_rotate[2] * consts::PI / 180f64).cos();
                    matrix_b[0][1] = (modeling_robo.modeling[i].d_rotate[2] * consts::PI / 180f64).sin() * -1f64;
                    matrix_b[1][0] = matrix_b[0][1] * -1f64;
                    matrix_b[1][1] = matrix_b[0][0];
                    cal_matrix(&modeling_robo.modeling[i].d_trans_matrix, &matrix_b)
                },
                1 => {
                    let mut matrix_b = cal_matrix_unit();
                    matrix_b[0][0] = (modeling_robo.modeling[i].d_rotate[1] * consts::PI / 180f64).cos();
                    matrix_b[0][2] = (modeling_robo.modeling[i].d_rotate[1] * consts::PI / 180f64).sin();
                    matrix_b[2][0] = matrix_b[0][2] * -1f64;
                    matrix_b[2][2] = matrix_b[0][0];
                    cal_matrix(&modeling_robo.modeling[i].d_trans_matrix, &matrix_b)
                },
                2 => {
                    let mut matrix_b = cal_matrix_unit();
                    matrix_b[1][1] = (modeling_robo.modeling[i].d_rotate[0] * consts::PI / 180f64).cos();
                    matrix_b[1][2] = (modeling_robo.modeling[i].d_rotate[0] * consts::PI / 180f64).sin() * -1f64;
                    matrix_b[2][1] = matrix_b[1][2] * -1f64;
                    matrix_b[2][2] = matrix_b[1][1];
                    cal_matrix(&modeling_robo.modeling[i].d_trans_matrix, &matrix_b)
                },
                3 => {
                    let mut matrix_b = cal_matrix_unit();
                    matrix_b[0][0] = modeling_robo.modeling[i].d_scale[0];
                    matrix_b[1][1] = modeling_robo.modeling[i].d_scale[1];
                    matrix_b[2][2] = modeling_robo.modeling[i].d_scale[2];
                    cal_matrix(&modeling_robo.modeling[i].d_trans_matrix, &matrix_b)
                }
                _ => panic!(),
            }
        }

        for j in 0..modeling_robo.robo_stl_model[i].n_stl_num as usize {
            modeling_robo.robo_stl_model[i].stl.push(
                Stl { 
                    pos: [
                        cal_pos(&modeling_robo.modeling[i].d_trans_matrix, &stl_model.stl[j].pos[0]),
                        cal_pos(&modeling_robo.modeling[i].d_trans_matrix, &stl_model.stl[j].pos[1]),
                        cal_pos(&modeling_robo.modeling[i].d_trans_matrix, &stl_model.stl[j].pos[2]),
                    ],
                    normal_vec: Pos::new(),
                }
            );
            modeling_robo.robo_stl_model[i].stl[j].normal_vec = 
                cal_normal_vec(&modeling_robo.robo_stl_model[i].stl[j]);
        }
        
    }
}