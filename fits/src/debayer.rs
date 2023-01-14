use ndarray::{Array, Ix3, s};
use crate::load_fits::ParsedFitsFileHDU;


pub fn debayer(hdu: &ParsedFitsFileHDU) -> Option<Array<f32, Ix3>> {
    let _bayer_pattern = hdu.bayer_pattern().unwrap();
    let array = hdu.data_copy_f32()?
        //.slice(ndarray::s![0..8, 0..8]).to_owned()
        ;
    /*
        How many virtual pixels (v_pixels) are there?
        R G R G
         v v v
        G B G B
         v v v
        R G G B
        this 3x4 matrix has 4 pixels (2 x 3)
        we are counting the junction of 4 surrounding pixels as a single v_pixel
        each v_pixel is surrounded by Gx2, Rx1 and Bx1
    */

    let shape = array.shape();
    if shape.len() == 3 {
        return Some(array.slice(s![.., .., ..]).to_owned());
    }
    let rgb_shape: [usize; 3] = [shape[0] - 1, shape[1] - 1, 3];

    //log::info!("v_pixel shape {:?}", rgb_shape);

    let mut rgb_array = ndarray::Array::<f32, ndarray::Ix3>::zeros(rgb_shape);
    let mut x = 0usize;
    let mut x_is_even = true;

    while x < rgb_shape[0] {
        let mut y = 0usize;
        let mut y_is_even = true;

        //x, y are constants here before entering the loop
        //but we have it here for visual pattern recognition
        //for when we do shifting in the loop below
        let mut surrounding_clock_wise: [f32; 4] = [
            array[[x + 0usize, y + 0usize]] as f32, //top-left
            array[[x + 0usize, y + 1usize]] as f32, //top-right
            array[[x + 1usize, y + 1usize]] as f32, //bottom-right
            array[[x + 1usize, y + 0usize]] as f32, //bottom-left
        ];

        while y < rgb_shape[1] {
            //top-right
            surrounding_clock_wise[1] = array[[x + 0, y + 1]] as f32;
            //bottom-right
            surrounding_clock_wise[2] = array[[x + 1, y + 1]] as f32;
            //top-left and bottom-left were handled in the closing of the previous iteration

            /*
                R G R G
                 v v v
                G B G B
                 v v v
                R G R B


                notice how a v is always surrounded by "RGBG" (notice its now RGGB) in the clockwise direction,
                so we just need to figure out an offset to start looking up surrounding_clock_wise
                r
             */

            let r_offset: usize =
                if x_is_even { if y_is_even { 0 } else { 1 } } else { if y_is_even { 3 } else { 2 } };

            let rr = surrounding_clock_wise[(r_offset + 0) % 4];
            let g0 = surrounding_clock_wise[(r_offset + 1) % 4];
            let bb = surrounding_clock_wise[(r_offset + 2) % 4];
            let g1 = surrounding_clock_wise[(r_offset + 3) % 4];

            rgb_array[[x, y, 0]] = rr;
            rgb_array[[x, y, 1]] = (g0 + g1) * 0.5;
            rgb_array[[x, y, 2]] = bb;

            y = y + 1;
            y_is_even = !y_is_even;
            //top-right, becomes top-left for the next iteration
            surrounding_clock_wise[0] = surrounding_clock_wise[1];
            //bottom-right, becomes bottom-left for the next iteration
            surrounding_clock_wise[3] = surrounding_clock_wise[2];
        }
        x = x + 1;
        x_is_even = !x_is_even;
    }

    return Some(rgb_array);
}
