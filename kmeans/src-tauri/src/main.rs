#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use rand::Rng;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rand_points, kmeans])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 随机生成样本
#[tauri::command]
fn rand_points(num: u32) -> Vec<(f64, f64)> {
    let mut points: Vec<(f64, f64)> = vec![];

    // let bases = [(-50., 50.), (50., 50.), (50., -50.), (-50., -50.)];
    // bases.iter().for_each(|&base| {
    //     rand_points_around_point(10, base)
    //         .iter()
    //         .for_each(|&point| {
    //             points.push(point);
    //         });
    // });

    let mut rang = rand::thread_rng();
    for _ in 0..num {
        points.push((
            rang.gen_range(-100_f64..100_f64),
            rang.gen_range(-100_f64..100_f64),
        ))
    }

    points
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

// kmeans算法实现
#[allow(dead_code)]
#[tauri::command]
fn kmeans(k: u32, points: Vec<(f64, f64)>, max: u32) -> HashMap<u32, Vec<(f64, f64)>> {
    // 聚类信息
    // 聚类ID: (聚类标准特征值, 聚类内的样本集合)
    let mut cluster_info: HashMap<u32, ((f64, f64), Vec<(f64, f64)>)> = HashMap::new();
    for i in 0..k {
        cluster_info.insert(i, (*points.get(i as usize).unwrap(), vec![]));
    }

    // 初始k个聚类的特征值
    for _ in 0..max {
        // 清空聚类内的样本集合
        cluster_info
            .iter_mut()
            .for_each(|(_, info)| info.1 = vec![]);

        // 判断每个样本和所属的类别
        points.iter().for_each(|point| {
            let mut min_similarity: Option<f64> = None;
            let mut most_similar_cluster_points: &mut Vec<(f64, f64)> = &mut vec![];

            // 寻找样本最相似的聚类
            cluster_info.iter_mut().for_each(|(_, info)| {
                // 样本与当前聚类的相似度
                let similarity =
                    ((info.0 .0 - point.0).powi(2) + (info.0 .1 - point.1).powi(2)).sqrt();

                match min_similarity {
                    Some(min) => {
                        if min > similarity {
                            min_similarity = Some(similarity);
                            most_similar_cluster_points = &mut info.1;
                        }
                    }
                    None => {
                        min_similarity = Some(similarity);
                        most_similar_cluster_points = &mut info.1;
                    }
                }
            });

            // 放入对应的类别
            most_similar_cluster_points.push(*point);
        });

        // 重新计算聚类特征值
        cluster_info.iter_mut().for_each(|(_, info)| {
            let mut x_count: f64 = 0.;
            let mut y_count: f64 = 0.;
            info.1.iter().for_each(|f| {
                x_count += f.0;
                y_count += f.1;
            });
            info.0 = (x_count / info.1.len() as f64, y_count / info.1.len() as f64);
        });
    }

    // 对样本进行分类
    let mut result: HashMap<u32, Vec<(f64, f64)>> = HashMap::new();
    cluster_info.iter().for_each(|(k, v)| {
        result.insert(*k, v.1.to_vec());
    });

    result
}
