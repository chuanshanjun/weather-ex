use exitfailure::ExitFailure;
use structopt::StructOpt;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct W {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Main,
}

impl W {
    async fn get(city: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid=aac5799559f931e8b9704f2695dcd45b", city);
        let url = Url::parse(&*url)?;
        let resp = reqwest::get(url).await?.json::<W>().await?;
        Ok(resp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(StructOpt)]
struct Input {
    city: String,
}

#[tokio::main]

async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    let resp = W::get(&input.city).await?;
    println!("{} \n 天气情况: {} \n 当前温度: {} \n 最高温度: {} \n 最低温度: {} \n 湿度: {}", input.city, resp.weather.details.main ,resp.main.temp, resp.main.temp_max, resp.main.temp_min, resp.main.humidity);
    Ok(())
}
