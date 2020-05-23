fn t_or_f(x1: i32, ps1: &Vec<i32>) -> bool {
    for p1 in ps1 {                   
        if p1 * p1 > x1 {                                      
             break;                                            
            } else if x1 % p1 == 0 {                               
              return  false;                                     
        }                                                  
    }                                                          
    true                                                                         
}                                                                                

fn prime(n1: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut ps2 = vec![2];
    let mut x2 = 3;

    if n1 <= 1 {
        Err(format!("NOT FOUND THIS {} FIELD", n1).into())
    } else {
        while x2 <= n1 {
            if t_or_f(x2, &ps2) {
                ps2.push(x2);
            }
            x2 += 2;
        }
        println!("{:?}", &ps2);
        Ok(())
    }
}


fn find_prime() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args().nth(1)
        .ok_or_else(|| "NO INSET VALUES".to_string())?;
    let p = arg.parse()?;
    prime(p)
}
fn main() {
    if let Err(e) = find_prime() {
        eprintln!("Error: {}", e);
    }
}
