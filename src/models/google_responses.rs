use models::gspots::{NewGSpot};

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugLog {
    pub line: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub formatted_address: String,
    pub name: String,
    pub place_id: String,
    pub types: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleResponse {
    pub candidates: Vec<Candidate>,
    //pub debug_log: DebugLog,
    pub status: String
}

impl Candidate{
    pub fn to_gspot(from: &Candidate) -> NewGSpot{
        let new_gspot = NewGSpot {
            name: from.name.as_str().to_string(),
            address: from.formatted_address.as_str().to_string(),
            google_place_id: from.place_id.as_str().to_string(),
            lat: 40.3,
            lng: -8.4,
            primary_type: from.types[0].as_str().to_string(),
            secondary_type: "Odeio a minha vida".to_string()
        };

        new_gspot
    }
}
