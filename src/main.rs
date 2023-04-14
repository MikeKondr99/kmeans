use k_means::*;
use std::{path::Path, collections::HashMap};
use to_vec::{ToVecResult};
use anyhow::Result;
use serde::de::DeserializeOwned;


#[derive(serde::Deserialize)]
pub struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    class: String,
}

impl From<&Iris> for [f64;4] {

    #[inline]
    fn from(value: &Iris) -> Self {
        [value.sepal_length,value.sepal_width,value.petal_length,value.petal_width]
    }
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
fn main() -> Result<()> {
    let data = read_data("data/iris-150.csv")?;
    let res = cluster_v4(&data[..], 3.try_into()?);
    let frequencies = res
          .fold(HashMap::new(), |mut map, (c,iris)|{
              map.entry((c,iris.class.clone()))
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              map
          });
    println!("{:<22} {:<15} {:<15} {:<15}",' ',0,1,2);
    for class in ["Iris-setosa","Iris-versicolor","Iris-virginica"] {
        print!("{:<20} | ",class);
        for i in 0..3 {
            print!("{:<15}",frequencies.get(&(i,class.to_owned())).unwrap_or(&0));
        }
        println!();
    }
    Ok(())
}
