use geo::{Destination, Distance, Geodesic, Point};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::{BufRead, Write};
use std::{fs::File, io};
fn main() {
    const DATA_IN: &str = "../data/good_trace";
    const DATA_OUT: &str = "../data/obfuscated";
    let file = File::open(DATA_IN).expect("error");
    let reader = io::BufReader::new(file);
    let mut lat_lon_ts: Vec<LatLonTs> = Vec::new();
    for line in reader.lines() {
        // Unwrap the line (ignoring errors).
        let line = line.expect("Could not read line");

        // Split the line by whitespace and collect into a vector.
        let parts: Vec<&str> = line.split(',').collect();

        // Parse the parts into the appropriate types and create a tuple.
        if parts.len() == 3 {
            if let (Ok(a), Ok(b), Ok(c)) = (
                parts[0].trim().parse::<f64>(),
                parts[1].trim().parse::<f64>(),
                parts[2].trim().parse::<u64>(),
            ) {
                lat_lon_ts.push(LatLonTs {
                    lat: a,
                    lon: b,
                    ts: c,
                });
            }
        }
    }
    let radiuses: Vec<f64> = vec![0.0, 50.0, 100.0, 200.0, 400.0, 800.0, 1600.0];
    let time_deltas: Vec<u64> = vec![0, 60, 300, 600, 1800, 3600];

    for radius in &radiuses {
        for time_delta in &time_deltas {
            let mut struct_obfuscator =
                LocationObfuscator::location_obfuscator(*radius, *time_delta);
            let mut instance_res: Vec<LatLonTs> = vec![];
            for loc in &lat_lon_ts {
                if let Some(obfuscated_loc) = struct_obfuscator.obfuscate_location(&loc) {
                    instance_res.push(obfuscated_loc);
                }
            }
            let blobs = struct_obfuscator.get_blobs();
            // Write instance_res to file
            let res_file_name = format!("{}/{}_{}_res", DATA_OUT, radius, time_delta);
            let mut res_file = File::create(res_file_name).expect("Could not create file");
            for loc in &instance_res {
                writeln!(res_file, "{},{},{}", loc.lat, loc.lon, loc.ts)
                    .expect("Could not write to file");
            }

            // Write blobs to file
            let blobs_file_name = format!("{}/{}_{}_blobs", DATA_OUT, radius, time_delta);
            let mut blobs_file = File::create(blobs_file_name).expect("Could not create file");
            for blob in blobs {
                writeln!(blobs_file, "{},{},{}", blob.lat, blob.lon, blob.radius)
                    .expect("Could not write to file");
            }
            println!("{:?}", blobs.len())
        }
    }
}
#[derive(Clone, Debug)]
struct PrivacyBlob {
    lat: f64,
    lon: f64,
    radius: f64,
}
#[derive(Clone, Debug)]
struct LatLonTs {
    lat: f64,
    lon: f64,
    ts: u64,
}

struct LocationObfuscator {
    history_blobs: Vec<PrivacyBlob>,
    blob_radius: f64,
    delta_time: u64,
    last_location: LatLonTs,
    rng: StdRng,
}

impl LocationObfuscator {
    fn location_obfuscator(blob_radius: f64, delta_time: u64) -> LocationObfuscator {
        return LocationObfuscator {
            history_blobs: vec![],
            blob_radius,
            delta_time,
            last_location: LatLonTs {
                lat: 0.0,
                lon: 0.0,
                ts: 0,
            },
            rng: StdRng::seed_from_u64(42),
        };
    }
    fn obfuscate_location(&mut self, loc: &LatLonTs) -> Option<LatLonTs> {
        let loc_as_point = Point::from((loc.lon, loc.lat));

        let matched_blobs: Vec<&PrivacyBlob> = self
            .history_blobs
            .par_iter()
            .filter(|&report| {
                Geodesic::distance(Point::from((report.lon, report.lat)), loc_as_point)
                    < report.radius
            })
            .collect();

        let blob: &PrivacyBlob = if matched_blobs.is_empty() {
            let sampled_radius = if self.blob_radius == 0.0 {
                0.0
            } else {
                self.rng.random_range(0.0..self.blob_radius)
            };
            let sampled_angle = self.rng.random_range(0.0..360.0);
            let new_point = Geodesic::destination(loc_as_point, sampled_angle, sampled_radius);
            let new_blob = PrivacyBlob {
                lon: new_point.x(),
                lat: new_point.y(),
                radius: self.blob_radius,
            };
            self.history_blobs.push(new_blob.clone());
            self.history_blobs.last().unwrap()
        } else {
            matched_blobs
                .par_iter()
                .min_by(|a, b| {
                    let point_a = Point::from((a.lon, a.lat));
                    let point_b = Point::from((b.lon, b.lat));
                    let da = Geodesic::distance(loc_as_point, point_a);
                    let db = Geodesic::distance(loc_as_point, point_b);
                    return da.partial_cmp(&db).unwrap();
                })
                .unwrap()
        };

        if loc.ts < self.delta_time * 1000 + self.last_location.ts {
            return None;
        }

        let result = if Geodesic::distance(
            Point::from((self.last_location.lat, self.last_location.lon)),
            loc_as_point,
        ) < self.blob_radius
        {
            LatLonTs {
                lat: self.last_location.lat,
                lon: self.last_location.lon,
                ts: loc.ts,
            }
        } else {
            LatLonTs {
                lat: blob.lat,
                lon: blob.lon,
                ts: loc.ts,
            }
        };

        self.last_location = result.clone();

        Some(result)
    }
    fn get_blobs(&self) -> &Vec<PrivacyBlob> {
        return &self.history_blobs;
    }
}
