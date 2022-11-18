fn main() {

    let mut grid: Vec<Vec<usize>> = vec![];
    
    let x_max = 50;
    let y_max = 50;
    
    let mut x = 0;
    
    while x < x_max {
        
        grid.push(vec![]);
        
        let mut y = 0;
        
        while y < y_max {
            
            grid[x].push(x * x_max + y);
            
            y += 1;
        }
        
        println!("{:?}", grid[x]);
        
        x += 1;
    }
}
