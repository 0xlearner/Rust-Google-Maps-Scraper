use crate::models::viewport::Viewport;

pub fn generate_grid(viewport: &Viewport, rows: usize, cols: usize) -> Vec<Vec<(f64, f64)>> {
    let (ne_lat, ne_lng) = viewport.northeast;
    let (sw_lat, sw_lng) = viewport.southwest;

    let lat_step = (ne_lat - sw_lat) / (rows as f64);
    let lng_step = (ne_lng - sw_lng) / (cols as f64);

    let mut grid = Vec::with_capacity(rows);

    for i in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for j in 0..cols {
            let lat = sw_lat + (i as f64) * lat_step;
            let lng = sw_lng + (j as f64) * lng_step;
            row.push((lat, lng));
        }
        grid.push(row);
    }

    grid
}
