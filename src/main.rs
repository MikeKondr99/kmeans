use std::{error::Error, num::NonZeroUsize, path::Path, process, collections::HashMap};
use rand::seq::SliceRandom;
use to_vec::{ToVec, ToVecResult};

fn main() {
    if let Err(err) = run() {
        println!("[Error]: {}", err);
        process::exit(1);
    }
}

#[derive(serde::Deserialize)]
struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    class: String,
}

fn read_data<P: AsRef<Path>>(path: P) -> Result<Vec<Iris>, Box<dyn Error>> {
    Ok(csv::Reader::from_path(path)?
        .deserialize::<Iris>()
        .to_vec_result()?)
}

impl From<&Iris> for [f64;4] {

    #[inline]
    fn from(value: &Iris) -> Self {
        [value.sepal_length,value.sepal_width,value.petal_length,value.petal_width]
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let data = read_data("iris.csv")?;
    let res = cluster(&data[..], 3.try_into()?);
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






fn cluster<'a,T,const N:usize>(data: &'a [T], k: NonZeroUsize) -> impl Iterator<Item = (usize,&'a T)>
where 
    &'a T: Into<[f64;N]>
{
    let points: Vec<[f64;N]> = data.iter().map(|x| x.into()).to_vec();
    let k = k.get();
    let (mut iter_count, iter_max) = (0, 200);
    let mut rng = rand::thread_rng();
    let mut res = vec![0; data.len()]; // индексы кластеров для данных
    let mut means = points.choose_multiple(&mut rng, k).cloned().to_vec(); // центры кластеров
    let mut avgs = vec![[0.0;N]; k];
    let mut counts = vec![0; k];
    let mut unstable = true;
    while unstable && iter_count < iter_max {
        unstable = false;
        iter_count += 1;
        // меняем кластер на ближайший
        for (i, d) in points.iter().enumerate() {
            let c = means.iter().enumerate()
                .min_by(|(_, a), (_, b)| f64::total_cmp(&distance(a, d),&distance(b, d)))
                .unwrap().0;
            unstable |= res[i] != c;
            res[i] = c;
            // сразу на ходу подсчитываем среднее
            counts[c] += 1;
            for (i,avg) in avgs[c].iter_mut().enumerate() {
                let temp = 1.0 /counts[c] as f64;
                *avg = temp * d[i] + (1.0 - temp) * (*avg);
            }
        }
        // смещаем кластеры и очищаем данные
        std::mem::swap(&mut means,&mut avgs);
        avgs.fill([0.0;N]);
        counts.fill(0);
    }
    res.into_iter().zip(data)
}

fn distance<const N:usize>(a: &[f64;N], b: &[f64;N]) -> f64 {
    (0..N).map(|i| (a[i] - b[i]).powi(2)).sum()
}

