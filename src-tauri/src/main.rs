// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use statrs::distribution::Erlang;
use std::collections::BTreeMap;
// use statrs::distribution::{Erlang, Normal};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize)]
struct TableData {
    length1: f64,
    length2: f64,
    total_length: f64,
    ei: f64,
    es: f64,
    coeficient_capacity: f64,
    coeficient_capacity_k: f64,
    bar_state: bool,
    probability: f64,
}

#[tauri::command(rename_all = "snake_case")]
fn ejercicio_11(
    media1: f64,
    variaza1: f64,
    k: u64,
    media2: f64,
    number_simulation: i64,
    specification: f64,
    err: f64,
    replicas: u64,
) -> BTreeMap<usize, BTreeMap<usize, TableData>> {
    let mut return_data: BTreeMap<usize, BTreeMap<usize, TableData>> = BTreeMap::new();
    for l in 1..=replicas {
        let mut return_table: BTreeMap<usize, TableData> = BTreeMap::new();
        let normal = Normal::new(media1, variaza1).unwrap();
        let erlang = Erlang::new(k, media2).unwrap();
        let mut rgn = rand::thread_rng();
        let ei = specification - err;
        let es = specification + err;
        for i in 1..=number_simulation {
            let length1 = normal.sample(&mut rgn);
            let length2 = erlang.sample(&mut rgn);

            let total_length = length1 + length2;
            let bar_state: bool;

            if total_length < es && total_length > ei {
                bar_state = true;
            } else {
                bar_state = false;
            }

            let spec_range = es - ei;
            let collect_total_length: Vec<f64> = return_table
                .iter()
                .skip(1)
                .take(i as usize)
                .map(|(_, table_data)| table_data.total_length)
                .collect();

            let process_mean = mean(&collect_total_length);
            let process_std_dev = std_deviation(&collect_total_length);
            let coeficient_capacity = spec_range / (6.0 * process_std_dev);
            let coeficient_capacity_k =
                coeficient_capacity * (1.0 - (process_mean - ei).abs() / (0.5 * spec_range));

            let sum: f64 = return_table
                .iter()
                .skip(1)
                .take(i as usize)
                .map(|(_, table_data)| (table_data.bar_state as u8) as f64)
                .sum();
            let probability = sum / i as f64;

            return_table.insert(
                i as usize,
                TableData {
                    length1,
                    length2,
                    total_length,
                    ei,
                    es,
                    coeficient_capacity,
                    coeficient_capacity_k,
                    bar_state,
                    probability,
                },
            );
        }
        return_data.insert(l as usize, return_table);
    }

    return_data
}

fn mean(data: &[f64]) -> f64 {
    let sum = data.iter().sum::<f64>();
    let count = data.len();
    match count {
        positive if positive > 0 => sum / count as f64,
        _ => 0.0,
    }
}

fn std_deviation(data: &[f64]) -> f64 {
    match (mean(data), data.len()) {
        (data_mean, count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - *value;
                    diff * diff
                })
                .sum::<f64>()
                / count as f64;
            variance.sqrt()
        }
        _ => 0.0,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, ejercicio_11])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
