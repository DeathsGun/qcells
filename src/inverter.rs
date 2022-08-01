use crate::battery::Battery;
use crate::models::InverterStateResponse;

#[derive(Debug)]
pub struct Inverter {
    pub battery: Option<Battery>,
    pub array_power: f64,
    pub grid_power_import: f64,
    pub grid_power_export: f64,
    pub power: f64,
}

impl Inverter {
    pub(crate) fn from_inverter_state(state: InverterStateResponse) -> Inverter {
        let mut import = 0.0;
        let mut export = 0.0;
        if state.grid_power < 0.0 {
            import = state.grid_power.abs()
        } else {
            export = state.grid_power.abs()
        }
        let mut battery: Option<Battery> = None;
        if state.battery_voltage.is_some() {
            battery = Some(Battery {
                capacity: state.battery_capacity.unwrap(),
                temperature_high: state.battery_temperature_high.unwrap(),
                temperature_low: state.battery_temperature_low.unwrap(),
                voltage: state.battery_voltage.unwrap(),
            })
        }

        let array_power: f64 = state.first_array_power.unwrap_or(0.0) +
            state.second_array_power.unwrap_or(0.0) +
            state.third_array_power.unwrap_or(0.0) +
            state.fourth_array_power.unwrap_or(0.0) +
            state.fifth_array_power.unwrap_or(0.0) +
            state.sixth_array_power.unwrap_or(0.0) +
            state.seventh_array_power.unwrap_or(0.0) +
            state.eighth_array_power.unwrap_or(0.0) +
            state.ninth_array_power.unwrap_or(0.0) +
            state.tenth_array_power.unwrap_or(0.0) +
            state.eleventh_array_power.unwrap_or(0.0) +
            state.twelfth_array_power.unwrap_or(0.0);

        let power: f64 = state.first_inverter_power.unwrap_or(0.0) +
            state.second_inverter_power.unwrap_or(0.0) +
            state.third_inverter_power.unwrap_or(0.0);

        return Inverter {
            array_power,
            power,
            grid_power_export: export,
            grid_power_import: import,
            battery,
        };
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_inverters() {
        let client = Client::new(
            "guest",
            "solax123456",
        );
        let client = match client {
            Ok(client) => client,
            Err(err) => panic!("{}", err.message)
        };
        let sites = client.user.get_sites();
        let sites = match sites {
            Ok(sites) => sites,
            Err(err) => panic!("{}", err.message)
        };
        assert!(!sites.is_empty());
        for site in sites {
            let inverter = match site.get_inverter() {
                Ok(inv) => inv,
                Err(err) => panic!("{}", err.message)
            };
            println!("{:#?}", inverter);
            if inverter.battery.is_some() {
                let battery = inverter.battery.unwrap();
                assert_ne!(battery.capacity, 0.0);
                assert_ne!(battery.temperature_high, 0.0);
                assert_ne!(battery.temperature_low, 0.0);
            }
        }
    }
}
