#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::Rng;
use rand_distr::num_traits::ToPrimitive;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rand_points, get_line])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 随机生成样本
#[tauri::command]
fn rand_points() -> Vec<(f64, f64)> {
    let mut points: Vec<(f64, f64)> = vec![];
    let mut rang = rand::thread_rng();
    let a = rang.gen_range(-1.5_f64..1.5_f64);
    let b = rang.gen_range(0_f64..10.);
    for _ in 0..100 {
        let x = rang.gen_range(-100_f64..100.);
        points.push((x, a * x + b + rang.gen_range(-10_f64..10.)));
    }
    return points;
}

#[tauri::command]
fn get_line(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let (a, b) = gradient_descent(points, 100000, 0.0001);

    // 返回直线的两个顶点
    vec![(-100., a * -100. + b), (100., a * 100. + b)]
}

// 梯度下降法求解最小二乘算法
fn gradient_descent(points: Vec<(f64, f64)>, num_iter: u32, lr: f64) -> (f64, f64) {
    // y = ax + b
    let mut a = 0_f64;
    let mut b = 0_f64;

    // 迭代num_iter次修正a、b参数
    let len = points.len();
    for _ in 0..num_iter {
        let mut a_err_count = 0_f64;
        let mut b_err_count = 0_f64;

        // 求总的误差值
        points.iter().for_each(|point| {
            let y_hat = (a * point.0) + b;
            a_err_count += point.0 * (point.1 - y_hat);
            b_err_count += point.1 - y_hat;
        });

        // 计算梯度
        let hat = -(2. / len.to_f64().unwrap());

        // 根据梯度修正参数
        a -= lr * hat * a_err_count;
        b -= lr * hat * b_err_count;
    }

    (a, b)
}
