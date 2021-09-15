use structopt::StructOpt;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, StructOpt)]
#[structopt(name = "treasurebox", about = "A set of command line tools.")]
struct Opt {
    /// City to check weather for
    #[structopt(short, long)]
    city: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
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
    sea_level: Option<i32>,
    grnd_level: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
    gust: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    #[serde(alias = "type")]
    systype: i32,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

async fn get_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=a093903fd9568150850ddebd91de0d0d", city);
    // println!("{}", url);
    // let url: Url = Url::parse(&url).unwrap();
    // println!("{:#?}", url);
    // let resp = reqwest::get(url).await?.json::<WeatherResponse>().await?;
    // let resp = reqwest::get(url).await?.text().await?;
    // let wr: WeatherResponse = serde_json::from_str(&resp).unwrap();
    // println!("{:#?}", wr);
    // Ok(wr)
    Ok(reqwest::get(url).await?.json::<WeatherResponse>().await?)
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    match get_weather(&opt.city).await {
        Ok(resp) => println!("{:#?}", resp),
        Err(e) => println!("{:#?}", e)
    }
}
