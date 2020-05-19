fn t_or_f(x1: i64, ps1: &Vec<i64>) -> bool {
    for p1 in ps1 {                   
        if p1 * p1 > x1 {                                      
             break;                                            
            } else if x1 % p1 == 0 {                               
                false;                                     
        }                                                  
    }                                                          
    true                                                                         
}                                                                                

fn prime(n1: i64) {                                                               
    let mut ps2 = vec![2];                                                        
    let mut x2 = 3;    

    if n1 <= 1 {
        println!("NOT FOWND THIS {} FIELD",n1);        
    } else {
        while x2 <= n1 {                                                            
            if t_or_f(x2, &ps2) {                                                   
               ps2.push(x2);                                                       
            }                                                                       
            x2 += 2;                                                         
        }
        println!("{:?}", &ps2);
    }                                                  
}                                                                                   
                                                                                    
fn main() {                                                                         
    let arg: String = std::env::args().nth(1).unwrap();
 //   if arg.len() == 0 {  
//        println!("NO INSET VALUES");
//    } else {          
        let p: i64 = arg.parse().unwrap();                                              
        prime(p);     
//    }                                                                  
}
