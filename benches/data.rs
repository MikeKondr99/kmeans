use serde::de::DeserializeOwned;
use to_vec::*;
use std::path::Path;
use anyhow::Result;

#[derive(serde::Deserialize)]
pub struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    class: String,
}

pub fn read_data<P,T>(path: P) -> Result<Vec<T>> 
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    Ok(csv::Reader::from_path(path)?
        .deserialize::<T>()
        .to_vec_result()?)
}

impl From<&Iris> for [f64;4] {

    #[inline]
    fn from(value: &Iris) -> Self {
        [value.sepal_length,value.sepal_width,value.petal_length,value.petal_width]
    }
}