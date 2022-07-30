use chrono::{Datelike, Local, Timelike};
use crate::{BASE_URL, Error, Inverter, Result};
use crate::models::{InverterStateResponse, SiteResponse};

pub struct Site {
    pub id: u64,
    pub rated_power: f32,
    pub grid_power: f32,
    pub total_yield: f64,
    pub total_eps_energy: f32,
    pub month_eps_energy: f32,
    pub month_yield: f32,
    pub site_name: String,
    pub inverter_num: i16,
    pub serial_number: String,
    pub wifi_sn: String,
    pub today_yield: f32,
    pub year_yield: f64,
    pub year_eps_energy: f32,
    pub today_eps_energy: f32,
    pub(crate) token: String,
}

impl Site {
    pub(crate) fn from_response(id: &u64, token: String, serial_number: &str, wifi_sn: &str, response: SiteResponse) -> Site {
        return Site {
            id: *id,
            rated_power: response.rated_power,
            grid_power: response.grid_power,
            total_yield: response.total_yield,
            total_eps_energy: response.total_eps_energy,
            month_eps_energy: response.month_eps_energy,
            month_yield: response.month_yield,
            site_name: response.site_name,
            inverter_num: response.inverter_num,
            serial_number: serial_number.to_string(),
            wifi_sn: wifi_sn.to_string(),
            today_yield: response.today_yield,
            year_yield: response.year_yield,
            year_eps_energy: response.year_eps_energy,
            today_eps_energy: response.today_eps_energy,
            token,
        };
    }

    pub fn get_inverter(&self) -> Result<Inverter> {
        let now = Local::now();
        let current_time = format!("{}-{}-{} {}:{}:{}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second());

        let client = reqwest::blocking::Client::new();
        let request = client.post(format!("{}/device/getInverterFromRedis", BASE_URL))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
            .header("token", self.token.to_owned())
            .form(&[("inverterSN", self.serial_number.to_string()), ("currentTime", current_time), ("wifiSN", self.wifi_sn.to_string())]).build();

        let request = match request {
            Ok(req) => req,
            Err(err) => return Err(Error { message: err.to_string() })
        };
        let response = match client.execute(request) {
            Ok(resp) => resp,
            Err(err) => return Err(Error { message: err.to_string() })
        };
        if !response.status().is_success() {
            return Err(Error { message: format!("Received invalid status code {}", response.status().as_u16()) });
        }

        let response: reqwest::Result<InverterStateResponse> = response.json();
        let response = match response {
            Ok(resp) => resp,
            Err(err) => return Err(Error { message: err.to_string() }),
        };

        return Ok(Inverter::from_inverter_state(response));
    }
}
