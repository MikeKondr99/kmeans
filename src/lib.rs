use float_ord::FloatOrd;
use rand::seq::SliceRandom;
use smallvec::*;
use std::num::NonZeroUsize;
use to_vec::*;

pub fn cluster_v1<'a, T, const N: usize>(
    data: &'a [T],
    k: NonZeroUsize,
) -> impl Iterator<Item = (usize, &'a T)>
where
    &'a T: Into<[f64; N]>,
{
    let points: Vec<[f64; N]> = data.iter().map(|x| x.into()).to_vec();
    let (mut iter_count, iter_max) = (0, 200);
    let mut rng = rand::thread_rng();
    let mut res = vec![0; data.len()]; // индексы кластеров для данных
    let mut means = points.choose_multiple(&mut rng, k.get()).cloned().to_vec(); // центры кластеров
    let mut avgs = vec![[0.0; N]; k.get()];
    let mut counts = vec![0; k.get()];
    let mut unstable = true;
    while unstable && iter_count < iter_max {
        unstable = false;
        iter_count += 1;
        // меняем кластер на ближайший
        for (i, d) in points.iter().enumerate() {
            let c = means
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| f64::total_cmp(&distance(a, d), &distance(b, d)))
                .unwrap()
                .0;
            unstable |= res[i] != c;
            res[i] = c;
            // сразу на ходу подсчитываем среднее
            counts[c] += 1;
            for (i, avg) in avgs[c].iter_mut().enumerate() {
                let temp = 1.0 / counts[c] as f64;
                *avg = temp * d[i] + (1.0 - temp) * (*avg);
            }
        }
        // смещаем кластеры и очищаем данные
        std::mem::swap(&mut means, &mut avgs);
        avgs.fill([0.0; N]);
        counts.fill(0);
    }
    res.into_iter().zip(data)
}

pub fn cluster_v2<'a, T, const N: usize>(
    data: &'a [T],
    k: NonZeroUsize,
) -> impl Iterator<Item = (usize, &'a T)>
where
    &'a T: Into<[f64; N]>,
{
    let points: Vec<[f64; N]> = data.iter().map(|x| x.into()).to_vec();
    let (mut iter_count, iter_max) = (0, 200);
    let mut rng = rand::thread_rng();
    let mut res = vec![0; data.len()]; // индексы кластеров для данных
    let mut means = points.choose_multiple(&mut rng, k.get()).cloned().to_vec(); // центры кластеров
    let mut avgs = vec![[0.0; N]; k.get()];
    let mut counts = vec![0; k.get()];
    let mut unstable = true;
    while unstable && iter_count < iter_max {
        unstable = false;
        iter_count += 1;
        // меняем кластер на ближайший
        for (i, d) in points.iter().enumerate() {
            let (mut c, mut min) = (0, distance(d, &means[0]));
            for i in 1..means.len() {
                let cur = distance(&means[i], d);
                if cur < min {
                    c = i;
                    min = cur;
                }
            }
            unstable |= res[i] != c;
            res[i] = c;
            // сразу на ходу подсчитываем среднее
            counts[c] += 1;
            for (i, avg) in avgs[c].iter_mut().enumerate() {
                let temp = 1.0 / counts[c] as f64;
                *avg = temp * d[i] + (1.0 - temp) * (*avg);
            }
        }
        // смещаем кластеры и очищаем данные
        std::mem::swap(&mut means, &mut avgs);
        avgs.fill([0.0; N]);
        counts.fill(0);
    }
    res.into_iter().zip(data)
}

pub fn cluster_v3<'a, T, const N: usize>(
    data: &'a [T],
    k: NonZeroUsize,
) -> impl Iterator<Item = (usize, &'a T)>
where
    &'a T: Into<[f64; N]>,
{
    let points: Vec<[f64; N]> = data.iter().map(|x| x.into()).to_vec();
    let (mut iter_count, iter_max) = (0, 200);
    let mut rng = rand::thread_rng();
    let mut res = vec![0; data.len()]; // индексы кластеров для данных
    let mut means = points.choose_multiple(&mut rng, k.get()).cloned().to_vec(); // центры кластеров
    let mut avgs = vec![[0.0; N]; k.get()];
    let mut counts = vec![0; k.get()];
    let mut unstable = true;
    while unstable && iter_count < iter_max {
        unstable = false;
        iter_count += 1;
        // меняем кластер на ближайший
        for (i, d) in points.iter().enumerate() {
            let (mut c, mut min) = (0, distance(d, &means[0]));
            for i in 1..means.len() {
                let cur = distance(&means[i], d);
                if cur < min {
                    c = i;
                    min = cur;
                }
            }
            unstable |= res[i] != c;
            res[i] = c;
            counts[c] += 1;
            avgs[c].iter_mut().zip(d).for_each(|(a, b)| *a += b);
        }
        for i in 0..k.get() {
            avgs[i].iter_mut().for_each(|a| *a /= counts[i] as f64);
        }
        // смещаем кластеры и очищаем данные
        std::mem::swap(&mut means, &mut avgs);
        avgs.fill([0.0; N]);
        counts.fill(0);
    }
    res.into_iter().zip(data)
}

pub fn cluster_v4<'a, T, const N: usize>(
    data: &'a [T],
    k: NonZeroUsize,
) -> impl Iterator<Item = (usize, &'a T)>
where
    &'a T: Into<[f64; N]>,
{
    let points: Vec<[f64; N]> = data.iter().map(|x| x.into()).to_vec();
    let (mut iter_count, iter_max) = (0, 200);
    let mut rng = rand::thread_rng();
    let mut res = vec![0; data.len()]; // индексы кластеров для данных
    let mut means: SmallVec<[[f64; N]; 8]> =
        points.choose_multiple(&mut rng, k.get()).cloned().collect(); // центры кластеров
    let mut avgs: SmallVec<[[f64; N]; 8]> = smallvec![[0.0;N]; k.get()];
    let mut counts: SmallVec<[usize; 8]> = smallvec![0; k.get()];
    let mut unstable = true;
    while unstable && iter_count < iter_max {
        unstable = false;
        iter_count += 1;
        // меняем кластер на ближайший
        for (i, d) in points.iter().enumerate() {
            let (mut c, mut min) = (0, distance(d, &means[0]));
            for i in 1..means.len() {
                let cur = distance(&means[i], d);
                if cur < min {
                    c = i;
                    min = cur;
                }
            }
            unstable |= res[i] != c;
            res[i] = c;
            counts[c] += 1;
            avgs[c].iter_mut().zip(d).for_each(|(a, b)| *a += b);
        }
        for i in 0..k.get() {
            avgs[i].iter_mut().for_each(|a| *a /= counts[i] as f64);
        }
        // смещаем кластеры и очищаем данные
        std::mem::swap(&mut means, &mut avgs);
        avgs.fill([0.0; N]);
        counts.fill(0);
    }
    res.into_iter().zip(data)
}

fn distance<const N: usize>(a: &[f64; N], b: &[f64; N]) -> f64 {
    (0..N).map(|i| (a[i] - b[i]).powi(2)).sum()
}
