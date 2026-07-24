use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CountPointCSV {
    pub count_point_id: i32,
    pub region_id: i32,
    pub region_name: String,
    pub region_ons_code: String,
    pub local_authority_id: i32,
    pub local_authority_name: String,
    pub local_authority_code: String,
    pub road_name: String,
    pub road_category: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct RawCountCSV {
    pub count_point_id: i32,
    pub direction_of_travel: String,
    pub count_date: String,
    pub hour: u32,

    #[serde(rename = "pedal_cycles")]
    pub bicycles: u32,

    #[serde(rename = "two_wheeled_motor_vehicles")]
    pub motorcycles: u32,

    #[serde(rename = "cars_and_taxis")]
    pub cars: u32,

    #[serde(rename = "buses_and_coaches")]
    pub buses: u32,

    #[serde(rename = "LGVs")]
    pub lgvs: u32,

    #[serde(rename = "all_HGVs")]
    pub hgvs: u32,
}