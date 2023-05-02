#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::Mutex};

use itertools::Itertools;
use lazy_static::lazy_static;
use rand::Rng;
use serde::Serialize;

lazy_static! {
    static ref POINTS: Mutex<Vec<Point>> = Mutex::new(vec![]);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rand_points, knn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Clone)]
struct Color {
    color: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Clone)]
struct Point {
    value: [f64; 2],
    itemStyle: Color,
}

static COLORS: [&str; 4] = ["red", "blue", "gray", "green"];

// 随机生成样本
#[tauri::command]
fn rand_points(num: u32) -> Box<Vec<Point>> {
    // 清空样本记录
    POINTS.lock().unwrap().clear();

    // 随机生成4种样本
    let bases = [(-50., 50.), (50., 50.), (50., -50.), (-50., -50.)];
    bases.iter().enumerate().for_each(|(index, &base)| {
        rand_points_around_point(num, base)
            .iter()
            .for_each(|&point| {
                POINTS.lock().unwrap().push(Point {
                    value: [point.0, point.1],
                    itemStyle: Color {
                        color: COLORS.get(index).unwrap().to_string(),
                    },
                });
            });
    });

    Box::new(POINTS.lock().unwrap().to_vec())
}

fn rand_points_around_point(num: u32, point: (f64, f64)) -> Vec<(f64, f64)> {
    let mut points: Vec<(f64, f64)> = vec![];
    let mut rang = rand::thread_rng();
    let x_min = point.0 - 25.;
    let x_max = point.0 + 25.;
    let y_min = point.1 - 25.;
    let y_max = point.1 + 25.;
    for _ in 0..num {
        points.push((rang.gen_range(x_min..x_max), rang.gen_range(y_min..y_max)));
    }
    points
}

// knn算法
#[tauri::command]
fn knn(k: u32, sample: [f64; 2]) -> String {
    // 计算sample和每个样本的相似度并按照相似度排正序
    POINTS.lock().unwrap().sort_by(|a, b| {
        let a_similar = (a.value.get(0).unwrap() - sample.get(0).unwrap()).powi(2)
            + (a.value.get(1).unwrap() - sample.get(1).unwrap()).powi(2);
        let b_similar = (b.value.get(0).unwrap() - sample.get(0).unwrap()).powi(2)
            + (b.value.get(1).unwrap() - sample.get(1).unwrap()).powi(2);
        a_similar.partial_cmp(&b_similar).unwrap()
    });

    POINTS
        .lock()
        .unwrap()
        .iter()
        .take(k as usize) // 只判断前k个
        .into_group_map_by(|&p| &p.itemStyle.color) // 根据分类进行分组
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().fold(0, |acc, _x| acc + 1))) // 统计每个分组内的样本个数
        .collect::<HashMap<&String, u32>>()
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1)) // 找出样本最多的分组
        .unwrap()
        .0
        .into()
}
