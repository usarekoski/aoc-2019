pub fn solve1(input: Vec<String>) -> u32 {
    let pixels_in_layer = 25 * 6;
    let data: Vec<u32> = input[0].chars().map(|c| c.to_digit(10).unwrap()).collect();
    let num_layers = data.len() / pixels_in_layer;
    let mut layers: Vec<(u32, u32, u32)> = (0..num_layers).map(|_| (0, 0, 0)).collect();
    for (idx, digit) in data.iter().enumerate() {
        let layer = idx / pixels_in_layer;
        match *digit {
            0 => layers[layer].0 += 1,
            1 => layers[layer].1 += 1,
            2 => layers[layer].2 += 1,
            _ => (),
        }
    }

    let min_layer = layers.iter().min_by_key(|layer| layer.0).unwrap();
    min_layer.1 * min_layer.2
}

pub fn solve2(input: Vec<String>) {
    let width = 25;
    let height = 6;
    let pixels_in_layer = width * height;
    let data: Vec<u32> = input[0].chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut image: Vec<u32> = (0..pixels_in_layer).map(|_| 2).collect();
    for (idx, digit) in data.iter().enumerate() {
        let layer = idx / pixels_in_layer;
        let image_idx = idx - layer * pixels_in_layer;
        let pixel = &mut image[image_idx];
        if *pixel != 2 {
            continue;
        }
        match *digit {
            0 => *pixel = 0,
            1 => *pixel = 1,
            2 => (),
            _ => panic!("unexpected digit"),
        }
    }

    // print image
    for (idx, pixel) in image.iter().enumerate() {
        if idx % width == 0 {
            println!("");
        }

        let c = match pixel {
            0 => ' ',
            1 => '@',
            _ => panic!("unexpected digit"),
        };
        print!("{}", c);
    }
}
