use std::process::Command;

use rand::Rng;
use reqwest;
use tokio::time::interval;

fn Starjess(N: u128) -> usize {
    // формула Стерджесса
    ((1.0 + 1.322 * (N as f64).log10()).floor() as usize) as usize
}

fn build_distribution_range(temps: &Vec<f64>) -> (Vec<f64>, f64, f64, f64) {
    let mut p_range = Vec::new();
    let s = Starjess(temps.len().try_into().unwrap());

    let min_temp = temps.iter().fold(f64::MAX, |acc, x| acc.min(*x));
    let max_temp = temps.iter().fold(f64::MIN, |acc, x| acc.max(*x));

    let h = (max_temp - min_temp) / (s as f64);

    for i in 0..s {
        let mut ni = 0;
        for x in temps.iter() {
            if x > &(min_temp + i as f64 * h) && x < &(min_temp + (i + 1) as f64 * h) {
                ni += 1;
            }
        }
        p_range.push(ni as f64 / temps.len() as f64); 
    }

    (p_range, h, min_temp, max_temp) // ряд распределения и ширина интервала
}





fn generate_sample_from_distribution(p_range: &[f64], h: f64, min_temp: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.0..1.0); // Случайное число от 0 до 1
    let mut cumulative_probability = 0.0;

    // Итерируемся по интервалам и ищем тот, в который попадает случайное число
    for (i, &prob) in p_range.iter().enumerate() {
        cumulative_probability += prob;
        if r <= cumulative_probability {
            // Генерируем случайное значение внутри выбранного интервала
            let random_value_in_interval = rng.gen_range(i as f64 * h..(i + 1) as f64 * h);
            return min_temp + random_value_in_interval;
        }
    }

    // В случае, если что-то пошло не так, возвращаем минимальное значение температуры
    min_temp
}





#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let latitude = "37.7749"; // Replace with your latitude
    let longitude = "-122.4194"; // Replace with your longitude

    let url = "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&hourly=temperature_2m&past_days=14";


    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    let temperatures: Vec<f64> = response["hourly"]["temperature_2m"]
        .as_array()
        .unwrap()
        .iter()
        .map(|temp| temp.as_f64().unwrap())
        .collect();

    println!("{:?}", &temperatures);

    let (p_range, h, min_temp, max_temp) = build_distribution_range(&temperatures);

    let python_script = "script.py";


    // =================================================================================
    // ГЕНЕРАЦИЯ АНАЛОГИЧНОЙ ВЫБОРКИ
    // =================================================================================
    let mut my_temps = Vec::new();
    let mut rng = rand::thread_rng();
    

    for _ in 0..temperatures.len() {
        let temp = generate_sample_from_distribution(&p_range, h, min_temp);
        my_temps.push(temp);
    }

    println!("{:?}", &my_temps);

    let (my_p_range, my_h, my_min_temp, my_max_temp) = build_distribution_range(&my_temps);

    let output = Command::new("python")
        .arg(python_script)
        .arg(format!("{:?}", p_range))
        .arg(format!("{}", h))
        .arg(format!("{}", min_temp))
        .arg(format!("{}", max_temp))
        .arg(format!("{:?}", my_p_range))
        .arg(format!("{}", my_h))
        .arg(format!("{}", my_min_temp))
        .arg(format!("{}", my_max_temp))
        .output()
        .expect("Failed to execute Python script");

    if output.status.success() {
        println!("Script output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Script error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}