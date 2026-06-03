pub struct PdfCoordinates;

impl PdfCoordinates {
    pub fn flip_y(y: f64, page_height: f64) -> f64 {
        page_height - y
    }
}
