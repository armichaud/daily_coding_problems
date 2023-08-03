fn main() {
    let mut triangle: Vec<Vec<i32>> = vec![
        vec![1], 
        vec![2, 3], 
        vec![1, 5, 1], 
        vec![4, 1, 5, 1]
    ];
    let mut maxes = vec![0; triangle.len() - 1];
    for row in triangle.iter_mut().rev() {
        for i in 0..maxes.len() {
            row[i] += maxes[i];
        }
        maxes = get_maxes(&row);
    }
    println!("{}", triangle[0][0]);
}

fn get_maxes(row: &[i32]) -> Vec<i32> {
    let mut maxes = Vec::new();
    for i in 0..(row.len() - 1) {
        if row[i] > row[i + 1] {
            maxes.push(row[i]);
        } else {
            maxes.push(row[i + 1]);
        }
    }
    maxes
}