use crate::models::{InvertersResponse, LoginResponse, SiteResponse};
use crate::{BASE_URL, Result, Error, Site};

pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub country: String,
    pub token: String,
}

impl User {
    pub(crate) fn from_login(resp: &LoginResponse) -> User {
        return User {
            id: resp.user.id,
            username: resp.user.username.to_owned(),
            email: resp.user.email.to_owned(),
            country: resp.user.country.to_owned(),
            token: resp.token.to_owned(),
        };
    }

    pub fn get_site(&self, id: &u64) -> Result<Site> {
        let client = reqwest::blocking::Client::new();
        let request = client.post(format!("{}/userIndex/getSiteInfo", BASE_URL))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
            .header("token", &self.token)
            .form(&[("siteId", id)]).build();

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

        let site_info: SiteResponse = match response.json() {
            Ok(info) => info,
            Err(err) => return Err(Error { message: err.to_string() }),
        };

        let serial_numbers = match self.get_site_inverter_sn(&site_info.site_name) {
            Ok(sn) => sn,
            Err(err) => return Err(err)
        };

        return Ok(Site::from_response(
            id,
            self.token.to_string(),
            serial_numbers.0.as_str(),
            serial_numbers.1.as_str(),
            site_info,
        ));
    }

    pub fn get_site_inverter_sn(&self, site_name: &str) -> Result<(String, String)> {
        let client = reqwest::blocking::Client::new();
        let request = client.post(format!("{}/device/getPage", BASE_URL))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
            .header("token", &self.token)
            .form(&[
                ("siteName", site_name),
                ("isSub", "1"),
                ("size", "10"),
                ("current", "1")
            ]).build();
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

        let inverters: InvertersResponse = match response.json() {
            Ok(resp) => resp,
            Err(err) => return Err(Error { message: err.to_string() })
        };

        if inverters.rows.is_empty() {
            return Err(Error { message: format!("No inverters for site {} found", site_name) });
        }

        let inverter = match inverters.rows.get(0) {
            Some(inv) => inv,
            None => return Err(Error { message: format!("No inverters for site {} found", site_name) })
        };

        return Ok((inverter.sn.to_string(), inverter.wifi_sn.to_string()));
    }

    pub fn get_sites(&self) -> Result<Vec<Site>> {
        let client = reqwest::blocking::Client::new();
        let request = client.post(format!("{}/userIndex/getUserAllSiteId", BASE_URL))
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded;charset=UTF-8")
            .header("token", &self.token)
            .form(&[("userId", self.id)]).build();

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
        let site_ids: reqwest::Result<Vec<String>> = response.json();
        let site_ids = match site_ids {
            Ok(ids) => ids.iter().map(|x| x.parse::<u64>().expect("Int required")).collect::<Vec<_>>(),
            Err(err) => return Err(Error { message: err.to_string() })
        };
        let mut sites: Vec<Site> = Vec::new();
        for site_id in site_ids {
            match self.get_site(&site_id) {
                Ok(site) => sites.push(site),
                Err(err) => return Err(err)
            };
        }
        return Ok(sites);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_sites() {
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
            assert_ne!(site.id, 0);
        }
    }
}