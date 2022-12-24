mod ndarray_image_ext;
mod load_fits;
mod debayer;

use std::ops::Range;
use ndarray::{Array, NdProducer};
use ndarray_stats::QuantileExt;
use plotters::prelude::*;

use rustronomy_fits::Fits;
use ndarray_image_ext::{NDArrayExt};
use crate::debayer::debayer;
use crate::load_fits::{ParsedFitsFile};

pub fn range(center: usize, width: usize, max: usize) -> Range<usize> {
    let min: usize = (center as i64 - width as i64).max(0i64) as usize;
    let max = (center + width).min(max);
    min..max
}

pub fn smaller(array: &ndarray::ArrayView<f32, ndarray::Ix2>) -> ndarray::Array<f32, ndarray::Ix2> {
    let width = array.shape()[0];
    let height = array.shape()[1];

    ndarray::Array::<f32, ndarray::Ix2>::from_shape_fn((width/8, height/8), |loc|{
        let win = array.slice(ndarray::s![
                range(loc.0 * 8, 4, width),
                range(loc.1 * 8, 4, height)
            ]);

        let mean = win.mean().unwrap();

        if (loc.0 == loc.1) && (loc.0 % 100 == 0){
            log::trace!("bg@({}, {})", loc.0, loc.1);
        }
        mean
    })
}

pub fn detect_spikes(name: &str, array: &ndarray::ArrayView<f32, ndarray::Ix2>) {

    let width = array.shape()[0];
    let height = array.shape()[1];
    let bg_span = 32;
    log::debug!("bg-span = {}", bg_span);


    let background: ndarray::Array<f32, ndarray::Ix2> =
        ndarray::Array::<f32, ndarray::Ix2>::from_shape_fn((width, height), |loc|{
            let win = array.slice(ndarray::s![
                range(loc.0, bg_span, width),
                range(loc.1, bg_span, height)
            ]);
            let central = array[loc];
            let min = *win.min().unwrap();
            let mean = win.mean().unwrap();
            let max = *win.max().unwrap();
            if (loc.0 == loc.1) && (loc.0 % 100 == 0){
                log::trace!("bg@({}, {})", loc.0, loc.1);
            }

            if central > mean * 1.4 {
                // array.slice(ndarray::s![
                //     range(loc.0, bg_span/2, width),
                //     range(loc.1, bg_span/2, height)
                // ]).mean().unwrap() * 0.8
                mean * 1.3
            }
            else {
                central
            }
            //
            // if central <= mean {
            //     min
            // }
            // else {
            //     mean
            // }


        });

    let signal: Array<f32, ndarray::Ix2> = array - &background;

    // let mut a = array[[0, 0]];
    // let slow = array.map(|y| {
    //     a = (a * 0.99 + *y * 0.01) as f32;
    //     return a;
    // });

    //let above_mean = array.map(|y| (y - mean).clamp(0.0, *max - mean));
    //let slow_above_mean = slow.map(|y| y.clamp(mean, *max));


    if height == 1 {
        plot(format!("{}-raw", name), array.slice(ndarray::s![.., 0]));
        plot(format!("{}-bg", name), background.slice(ndarray::s![.., 0]));
        plot(format!("{}-signal", name), signal.slice(ndarray::s![.., 0]));
    }
    else {
        array.view().create_gray_image().save(format!("images/{}-raw.png", name)).unwrap();
        background.view().create_gray_image().save(format!("images/{}-bg.png", name)).unwrap();
        signal.view().create_gray_image().save(format!("images/{}-signal.png", name)).unwrap();
    }

}

pub fn plot(name: String, array: ndarray::ArrayView<f32, ndarray::Ix1>) {
    let shape = array.shape();
    let f_name = format!("images/{}.png", name);
    let drawing_area = BitMapBackend::new(&f_name, (600, 400))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let min = array.min().unwrap();
    let max = array.max().unwrap();

    let x = 0i32..(shape[0] as i32);
    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x.clone(), min.round()..max.round())
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(
        LineSeries::new(x.clone().map(|x| {
            let y = array[[x as usize]];
            (x, y)
        }), ShapeStyle {
            color: BLACK.to_rgba(),
            filled: false,
            stroke_width: 1
        }),
    ).unwrap();

}

pub fn fits() {
    let f = ParsedFitsFile::parse("/Users/k/astro/captures/HHetc-light2C-g94-2022-12-18-1671352856041.fits".as_ref()).unwrap();
    log::info!("{:?}", f);

    let hdu0 = f.hdu(0).unwrap();
    log::info!("{:?}", hdu0);

    let rgb_array = debayer(&hdu0).unwrap();
    let shape = rgb_array.shape();



    //let mut image = rgb_array.create_rgba_image(&StretchParams {scale: 0.3} );
    // imageproc::drawing::draw_filled_circle_mut(
    //     &mut image, (100, 100), 20, image::Rgba::from([255, 0, 0, 128]));

    //image.save("images/test0.png").unwrap();

    //rgb_array.create_r_image(&StretchParams {scale: 0.3}).save("images/test0-r.png").unwrap();
    //rgb_array.create_g_image(&StretchParams {scale: 0.3}).save("images/test0-g.png").unwrap();
    //rgb_array.create_b_image(&StretchParams {scale: 0.3}).save("images/test0-b.png").unwrap();
    //
    let r_array: ndarray::ArrayView<f32, ndarray::Ix2> = rgb_array.slice(ndarray::s![..,..,0]);
    r_array.view()
        .create_gray_image()
        .save("images/red-full.png").unwrap();

    let r_small = smaller(&r_array);
    r_small.view()
        .create_gray_image()
        .save("images/red-small.png").unwrap();
    //
    //let cols_sum = r_array.dot(&ndarray::Array::from_elem((r_array.shape()[1], 1), 1.0/r_array.shape()[1] as f32));
    //let row_sum = r_array.t().dot(&ndarray::Array::from_elem((r_array.shape()[0], 1), 1.0/r_array.shape()[0] as f32));

    //detect_spikes("cols", &cols_sum.view());
    //detect_spikes("rows", &row_sum.view());
    // log::info!("{:?}, {:?}", cols_sum.shape(), row_sum.shape());

    detect_spikes("red", &r_small.view());


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_test() {
        env_logger::init();
        fits();
    }
}
