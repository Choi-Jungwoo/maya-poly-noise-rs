use cpython::{PyResult, Python, py_module_initializer, py_fn, PyModule, ObjectProtocol, PyObject, NoArgs};
use rand::Rng;

py_module_initializer!(maya_poly_noise, |py, m| {
    m.add(py, "poly_noise", py_fn!(py, poly_noise_py(geo_object:PyObject, max_displacement: f64)))?;
    Ok(())
});

fn rand_point(max_displacement: f64) -> f64 {
    rand::thread_rng().gen::<f64>() * max_displacement
}

fn generate_m_point_array(py: Python, om: &PyModule, geo_iter: &PyObject, max_displacement: f64) -> PyResult<PyObject> {
    let point_array = om.call(py, "MPointArray", NoArgs, None)?;
    geo_iter.call_method(py, "allPositions", (&point_array, ), None)?;

    let length = point_array.call_method(py, "length", NoArgs, None)?.extract::<usize>(py)?;
    for index in 0..length {
        let point = point_array.call_method(py, "__getitem__", (index, ), None)?;
        point_array.call_method(
            py,
            "set",
            (
                index,
                rand_point(max_displacement) + point.getattr(py, "x")?.extract::<f64>(py)?,
                rand_point(max_displacement) + point.getattr(py, "y")?.extract::<f64>(py)?,
                rand_point(max_displacement) + point.getattr(py, "z")?.extract::<f64>(py)?,
                point.getattr(py, "w")?.extract::<f64>(py)?,
            ),
            None,
        )?;
    }

    Ok(point_array)
}

fn poly_noise_py(py: Python, geo_object: PyObject, max_displacement: f64) -> PyResult<&'static str> {
    let om = py.import("maya.OpenMaya")?;

    let selection = om.call(py, "MSelectionList", NoArgs, None)?;
    selection.call_method(py, "add", (&geo_object, ), None)?;

    let dag_path = om.call(py, "MDagPath", NoArgs, None)?;
    selection.call_method(py, "getDagPath", (0, &dag_path), None)?;

    let geo_iter = om.call(py, "MItGeometry", (&dag_path, ), None)?;
    let point_array = generate_m_point_array(py, &om, &geo_iter, max_displacement)?;

    geo_iter.call_method(py, "setAllPositions", (point_array, ), None)?;

    Ok("")
}