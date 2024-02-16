extern crate base64;
extern crate image;
extern crate wu_diff;

#[cfg(test)]
mod tests {

    use base64::engine::Engine;
    use base64::engine::general_purpose::STANDARD;
    use image::*;
    use wu_diff::*;

    #[test]
    fn image_diff_test() {
        let before = image::open("./tests/images/before.png").unwrap();
        let after = image::open("./tests/images/after.png").unwrap();
        let diff = diff(&create_encoded_rows(&before), &create_encoded_rows(&after));
        let mut added_indexes: Vec<usize> = Vec::new();
        let mut removed_indexes: Vec<usize> = Vec::new();
        for d in diff.iter() {
            match d {
                DiffResult::Added(a) => added_indexes.push(a.new_index.unwrap()),
                DiffResult::Removed(r) => removed_indexes.push(r.old_index.unwrap()),
                _ => (),
            }
        }
        assert_eq!(
            added_indexes,
            vec![
                74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 282, 283, 284, 285, 286, 287,
                288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303,
                304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319,
                320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 510, 511,
            ]
        )
    }

    fn create_encoded_rows(image: &DynamicImage) -> Vec<String> {
        image
            .as_bytes().to_vec()
            .chunks(image.dimensions().0 as usize * 4)
            .map(|chunk| STANDARD.encode(chunk))
            .collect()
    }
}
