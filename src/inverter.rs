use crate::battery::Battery;
use crate::models::InverterStateResponse;

#[derive(Debug)]
pub struct Inverter {
    pub battery: Option<Battery>,
    pub array_power: f64,
    pub grid_power_import: f32,
    pub grid_power_export: f32,
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
        return Inverter {
            array_power: (state.first_array_power + state.second_array_power) as f64,
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
            if inverter.battery.is_some() {
                let battery = inverter.battery.unwrap();
                assert_ne!(battery.voltage, 0.0);
                assert_ne!(battery.capacity, 0.0);
                assert_ne!(battery.temperature_high, 0.0);
                assert_ne!(battery.temperature_low, 0.0);
            }
        }
    }
}
