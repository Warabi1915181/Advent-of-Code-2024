use std::fs;

fn vertical(neighbour: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    (0..neighbour.len()).for_each(|x| {
        if neighbour[x][0] == 'X'
            && neighbour[x][1] == 'M'
            && neighbour[x][2] == 'A'
            && neighbour[x][3] == 'S'
        {
            count += 1;
        }
        if neighbour[x][0] == 'S'
            && neighbour[x][1] == 'A'
            && neighbour[x][2] == 'M'
            && neighbour[x][3] == 'X'
        {
            count += 1;
        }
    });
    count
}

fn horizontal(neighbour: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    (0..neighbour[0].len()).for_each(|y| {
        if neighbour[0][y] == 'X'
            && neighbour[1][y] == 'M'
            && neighbour[2][y] == 'A'
            && neighbour[3][y] == 'S'
        {
            count += 1;
        }
        if neighbour[0][y] == 'S'
            && neighbour[1][y] == 'A'
            && neighbour[2][y] == 'M'
            && neighbour[3][y] == 'X'
        {
            count += 1;
        }
    });
    count
}

fn diagonal(neighbour: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    if neighbour[0][0] == 'X'
        && neighbour[1][1] == 'M'
        && neighbour[2][2] == 'A'
        && neighbour[3][3] == 'S'
    {
        count += 1;
    }
    if neighbour[0][3] == 'X'
        && neighbour[1][2] == 'M'
        && neighbour[2][1] == 'A'
        && neighbour[3][0] == 'S'
    {
        count += 1;
    }
    if neighbour[0][0] == 'S'
        && neighbour[1][1] == 'A'
        && neighbour[2][2] == 'M'
        && neighbour[3][3] == 'X'
    {
        count += 1;
    }
    if neighbour[0][3] == 'S'
        && neighbour[1][2] == 'A'
        && neighbour[2][1] == 'M'
        && neighbour[3][0] == 'X'
    {
        count += 1;
    }
    count
}

fn apply_kernel<F>(neighbour: Vec<Vec<char>>, f: F, stride_x: usize, stride_y: usize) -> i32
where
    F: Fn(Vec<Vec<char>>) -> i32,
{
    let mut count = 0;
    for x in (0..neighbour.len() - 3).step_by(stride_x) {
        for y in (0..neighbour[0].len() - 3).step_by(stride_y) {
            let mut kernel: Vec<Vec<char>> = Vec::with_capacity(4);
            for i in 0..4 {
                kernel.push(Vec::with_capacity(4));
                for j in 0..4 {
                    kernel[i].push(neighbour[x + i][y + j]);
                }
            }
            count += f(kernel);
        }
    }
    if neighbour.len() % stride_x != 0 {
        let leftovers = neighbour.len() % stride_x;
        let x = neighbour.len() - leftovers;
        for y in (0..neighbour[0].len() - 3).step_by(stride_y) {
            let mut kernel: Vec<Vec<char>> = Vec::with_capacity(leftovers);
            for i in 0..leftovers {
                kernel.push(Vec::with_capacity(4));
                for j in 0..4 {
                    kernel[i].push(neighbour[x + i][y + j]);
                }
            }
            count += f(kernel);
        }
    }
    if neighbour.len() % stride_y != 0 {
        let leftovers = neighbour[0].len() % stride_y;
        for x in (0..neighbour.len() - 3).step_by(stride_x) {
            let y = neighbour[0].len() - leftovers;
            let mut kernel: Vec<Vec<char>> = Vec::with_capacity(4);
            for i in 0..4 {
                kernel.push(Vec::with_capacity(leftovers));
                for j in 0..leftovers {
                    kernel[i].push(neighbour[x + i][y + j]);
                }
            }
            count += f(kernel);
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let rows: Vec<Vec<char>> = lines
        .clone()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    if rows.len() >= 4 {
        count += apply_kernel(rows.clone(), horizontal, 1, 4)
    }
    if rows[0].len() >= 4 {
        count += apply_kernel(rows.clone(), vertical, 4, 1)
    }
    if rows.len() >= 4 && rows[0].len() >= 4 {
        count += apply_kernel(rows.clone(), diagonal, 1, 1)
    }
    println!("Count: {}", count);
}
