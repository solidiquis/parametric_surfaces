#[macro_export]
macro_rules! fmt_mat_f32 {
    ( $i:expr ) => (
        {
            let mat: [[f32; 4]; 4] = *$i.as_ref();
            mat.into_iter().flatten().collect::<Vec<f32>>()
        }
    );
}

#[macro_export]
macro_rules! buf_f32 {
    ( $i:expr ) => (
        {
            let slice: &[f32] = &$i[..];
            js_sys::Float32Array::from(slice).buffer()
        }
    );
}

#[macro_export]
macro_rules! buf_u16 {
    ( $i:expr ) => (
        {
            let slice: &[u16] = &$i[..];
            js_sys::Uint16Array::from(slice).buffer()
        }
    );
}
