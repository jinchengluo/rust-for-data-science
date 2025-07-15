use numpy::{PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyfunction]
#[pyo3(signature = (data, y, test_size=0.2, train_size=None))]
fn stratified_train_test_split<'py>(
    py: Python<'py>,
    data: PyReadonlyArray2<'py, f64>,
    y: PyReadonlyArray1<'py, i32>,
    test_size: f64,
    train_size: Option<f64>,
) -> PyResult<(
    Bound<'py, PyArray2<f64>>,
    Bound<'py, PyArray2<f64>>,
    Bound<'py, PyArray1<i32>>,
    Bound<'py, PyArray1<i32>>,
)> {
    // Convert numpy arrays to Rust arrays
    let data = data.as_array();
    let y = y.as_array();

    // Parameters
    let n_samples = data.shape()[0];
    let n_features = data.shape()[1];

    // Get unique labels
    let labels: Vec<i32> = {
        use std::collections::HashSet;
        let unique_set: HashSet<i32> = y.iter().cloned().collect();
        let mut labels: Vec<i32> = unique_set.into_iter().collect();
        labels.sort();
        labels
    };

    let train_input = train_size.is_some();

    let mut x_train_set: Vec<Vec<f64>> = Vec::new();
    let mut x_test_set: Vec<Vec<f64>> = Vec::new();
    let mut y_train_set: Vec<i32> = Vec::new();
    let mut y_test_set: Vec<i32> = Vec::new();

    // Construct subsets - group samples by label
    let mut sub_label_sets: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();
    for &label in &labels {
        sub_label_sets.insert(label, Vec::new());
    }

    for i in 0..n_samples {
        let label = y[i];
        let sample: Vec<f64> = (0..n_features).map(|j| data[[i, j]]).collect();
        sub_label_sets.get_mut(&label).unwrap().push(sample);
    }

    // Construct train and test sets
    let mut sub_test_sets: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();
    let mut sub_train_sets: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();

    for &label in &labels {
        sub_test_sets.insert(label, Vec::new());
        sub_train_sets.insert(label, Vec::new());
    }

    for &label in &labels {
        let sub_label_set = sub_label_sets.get(&label).unwrap();
        let n_samples_sub_label_set = sub_label_set.len();
        let mut test_prop = 0.0;
        let mut train_prop = 0.0;

        for sample in sub_label_set {
            if test_prop <= test_size {
                sub_test_sets.get_mut(&label).unwrap().push(sample.clone());
                test_prop += 1.0 / n_samples_sub_label_set as f64;
                y_test_set.push(label);
            } else {
                if train_input {
                    if let Some(train_size_val) = train_size {
                        if train_prop > train_size_val {
                            break;
                        }
                    }
                }
                sub_train_sets.get_mut(&label).unwrap().push(sample.clone());
                train_prop += 1.0 / n_samples_sub_label_set as f64;
                y_train_set.push(label);
            }
        }
    }

    // Combine all label subsets
    for &label in &labels {
        x_train_set.extend(sub_train_sets.get(&label).unwrap().clone());
        x_test_set.extend(sub_test_sets.get(&label).unwrap().clone());
    }

    // Convert back to numpy arrays
    let x_train_array = PyArray2::from_vec2(py, &x_train_set)?;
    let x_test_array = PyArray2::from_vec2(py, &x_test_set)?;
    let y_train_array = PyArray1::from_vec(py, y_train_set);
    let y_test_array = PyArray1::from_vec(py, y_test_set);

    Ok((x_train_array, x_test_array, y_train_array, y_test_array))
}

#[pyfunction]
#[pyo3(signature = (data, y, test_size=0.2, train_size=None))]
fn stratified_kfold_split<'py>(
    py: Python<'py>,
    data: PyReadonlyArray2<'py, f64>,
    y: PyReadonlyArray1<'py, i32>,
    test_size: f64,
    train_size: Option<f64>,
) -> PyResult<(
    Bound<'py, PyArray2<f64>>,
    Bound<'py, PyArray2<f64>>,
    Bound<'py, PyArray1<i32>>,
    Bound<'py, PyArray1<i32>>,
)> {
    // Convert numpy arrays to Rust arrays
    let data = data.as_array();
    let y = y.as_array();

    // Parameters
    let n_samples = data.shape()[0];
    let n_features = data.shape()[1];

    // Get unique labels
    let labels: Vec<i32> = {
        use std::collections::HashSet;
        let unique_set: HashSet<i32> = y.iter().cloned().collect();
        let mut labels: Vec<i32> = unique_set.into_iter().collect();
        labels.sort();
        labels
    };

    let train_input = train_size.is_some();

    let mut x_train_set: Vec<Vec<f64>> = Vec::new();
    let mut x_test_set: Vec<Vec<f64>> = Vec::new();
    let mut y_train_set: Vec<i32> = Vec::new();
    let mut y_test_set: Vec<i32> = Vec::new();

    // Construct subsets - group samples by label
    let mut sub_label_sets: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();
    for &label in &labels {
        sub_label_sets.insert(label, Vec::new());
    }

    for i in 0..n_samples {
        let label = y[i];
        let sample: Vec<f64> = (0..n_features).map(|j| data[[i, j]]).collect();
        sub_label_sets.get_mut(&label).unwrap().push(sample);
    }

    // Construct train and test sets
    let mut sub_test_sets: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();
    let mut sub_train_sets: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();

    for &label in &labels {
        sub_test_sets.insert(label, Vec::new());
        sub_train_sets.insert(label, Vec::new());
    }

    for &label in &labels {
        let sub_label_set = sub_label_sets.get(&label).unwrap();
        let n_samples_sub_label_set = sub_label_set.len();
        let mut test_prop = 0.0;
        let mut train_prop = 0.0;

        for sample in sub_label_set {
            if test_prop <= test_size {
                sub_test_sets.get_mut(&label).unwrap().push(sample.clone());
                test_prop += 1.0 / n_samples_sub_label_set as f64;
                y_test_set.push(label);
            } else {
                if train_input {
                    if let Some(train_size_val) = train_size {
                        if train_prop > train_size_val {
                            break;
                        }
                    }
                }
                sub_train_sets.get_mut(&label).unwrap().push(sample.clone());
                train_prop += 1.0 / n_samples_sub_label_set as f64;
                y_train_set.push(label);
            }
        }
    }

    // Combine all label subsets
    for &label in &labels {
        x_train_set.extend(sub_train_sets.get(&label).unwrap().clone());
        x_test_set.extend(sub_test_sets.get(&label).unwrap().clone());
    }

    // Convert back to numpy arrays
    let x_train_array = PyArray2::from_vec2(py, &x_train_set)?;
    let x_test_array = PyArray2::from_vec2(py, &x_test_set)?;
    let y_train_array = PyArray1::from_vec(py, y_train_set);
    let y_test_array = PyArray1::from_vec(py, y_test_set);

    Ok((x_train_array, x_test_array, y_train_array, y_test_array))
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stratified_train_test_split, m)?)?;
    m.add_function(wrap_pyfunction!(stratified_kfold_split, m)?)?;
    Ok(())
}
