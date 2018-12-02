use models::gspots::NewGSpot;

#[derive(Debug, Serialize, Deserialize)]
pub struct Photos {
    pub height: i32,
    pub html_attributions: Vec<String>,
    pub photo_reference: String,
    pub width: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewport {
    pub northeast: Point,
    pub southwest: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub location: Point,
    pub viewport: Viewport,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub formatted_address: String,
    pub geometry: Geometry,
    pub name: String,
    pub photos: Option<Vec<Photos>>,
    pub place_id: String,
    pub types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleResponse {
    pub candidates: Vec<Candidate>,
    //pub debug_log: DebugLog,
    pub status: String,
}

impl Candidate {
    pub fn to_gspot(from: &Candidate) -> NewGSpot {
        let new_gspot = NewGSpot {
            name: from.name.as_str().to_string(),
            address: from.formatted_address.as_str().to_string(),
            google_place_id: from.place_id.as_str().to_string(),
            lat: from.geometry.location.lat,
            lng: from.geometry.location.lng,
            primary_type: from.types[0].as_str().to_string(),
            secondary_type: "Odeio a minha vida".to_string(),
        };

        new_gspot
    }

    pub fn get_photo_id(from: &Candidate) -> String {
        match from.photos {
            // The division was valid
            Some(ref x) => x[0].photo_reference.as_str().to_string(),
            // The division was invalid
            None => "error".to_string(),
        }
    }
}
