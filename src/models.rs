use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub(crate) struct Login {
    pub(crate) username: String,
    #[serde(rename(serialize = "userpwd"))]
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub code: i8,
    pub success: bool,
    pub token: String,
    pub(crate) user: UserInfo,
}

#[derive(Deserialize)]
pub(crate) struct UserInfo {
    pub id: u64,
    #[serde(rename(deserialize = "loginName"))]
    pub username: String,
    pub email: String,
    pub country: String,
}

#[derive(Deserialize)]
pub(crate) struct SiteResponse {
    #[serde(rename(deserialize = "ratedPower"))]
    pub rated_power: f32,
    #[serde(rename(deserialize = "gridPower"))]
    pub grid_power: f32,
    #[serde(rename(deserialize = "totalYield"))]
    pub total_yield: f64,
    #[serde(rename(deserialize = "totalEpsEnergy"))]
    pub total_eps_energy: f32,
    #[serde(rename(deserialize = "monthEpsEnergy"))]
    pub month_eps_energy: f32,
    #[serde(rename(deserialize = "monthYield"))]
    pub month_yield: f32,
    #[serde(rename(deserialize = "siteName"))]
    pub site_name: String,
    #[serde(rename(deserialize = "inverterNum"))]
    pub inverter_num: i16,
    #[serde(rename(deserialize = "todayYield"))]
    pub today_yield: f32,
    #[serde(rename(deserialize = "yearYield"))]
    pub year_yield: f64,
    #[serde(rename(deserialize = "yearEpsEnergy"))]
    pub year_eps_energy: f32,
    #[serde(rename(deserialize = "todayEpsEnergy"))]
    pub today_eps_energy: f32,
}

#[derive(Deserialize)]
pub(crate) struct InverterStateResponse {
    #[serde(rename(deserialize = "batVoltage1"))]
    pub battery_voltage: Option<f64>,
    #[serde(rename(deserialize = "batteryCapacity"))]
    pub battery_capacity: Option<f32>,
    #[serde(rename(deserialize = "batteryCellTemperatureHigh"))]
    pub battery_temperature_high: Option<f32>,
    #[serde(rename(deserialize = "batteryCellTemperatureLow"))]
    pub battery_temperature_low: Option<f32>,
    // #[serde(rename(deserialize = "batteryHealth"))]
    // pub battery_health: String,
    #[serde(rename(deserialize = "feedinpower"))]
    pub grid_power: f64,
    #[serde(rename(deserialize = "powerdc1"))]
    pub first_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc2"))]
    pub second_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc3"))]
    pub third_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc4"))]
    pub fourth_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc5"))]
    pub fifth_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc6"))]
    pub sixth_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc7"))]
    pub seventh_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc8"))]
    pub eighth_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc9"))]
    pub ninth_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc10"))]
    pub tenth_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc11"))]
    pub eleventh_array_power: Option<f64>,
    #[serde(rename(deserialize = "powerdc12"))]
    pub twelfth_array_power: Option<f64>,
    #[serde(rename(deserialize = "pac1"))]
    pub first_inverter_power: Option<f64>,
    #[serde(rename(deserialize = "pac2"))]
    pub second_inverter_power: Option<f64>,
    #[serde(rename(deserialize = "pac3"))]
    pub third_inverter_power: Option<f64>,
    //#[serde(rename(deserialize = "yieldtotal"))]
    //pub yield_total: u32
}

#[derive(Deserialize)]
pub(crate) struct InvertersResponse {
    pub rows: Vec<InverterInfo>
}

#[derive(Deserialize)]
pub(crate) struct InverterInfo {
    #[serde(rename = "wifiSN")]
    pub wifi_sn: String,
    pub sn: String,
}