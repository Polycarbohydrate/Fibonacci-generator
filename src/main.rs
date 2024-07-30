use num::bigint::BigInt;
use std::io;


fn main() {
    loop   {
        println!("--------------------------------------------------------------------------------------");
        println!("Type 1 if you want to see the Fibonacci Sequence Generated for infinity.");
        println!("^^^ Will crash the computer due to RAM usage after a period of time unless you exit. ^^^");
        println!("Type 2 to see the nth number of the sequence.");
        println!("--------------------------------------------------------------------------------------");
        
        let mut a = String::new();
        
        io::stdin().read_line(&mut a).expect("---Failed to read line---");
        
        let a: u32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("--------------");
        println!("You typed {a}.");
        println!("--------------");
        
        if a == 1   {
            gen();
            break
        }
        else if a == 2  {
            term();
            break
        }
        else    {
            println!("Please use only numbers, either 1 or 2.");
            println!("---------------------------------------");
            continue;
        }
    }
}

fn gen()   {
    let mut gen1 = BigInt::from(1);
    let mut gen2 = BigInt::from(1);
    let mut nextnum = gen2.clone();
    let hold = 0;
    println!("{gen1}");
    println!("{gen2}");
        while hold == 0 {
            gen1 = gen2;
            gen2 = nextnum;
            nextnum = gen1 + gen2.clone();
            println!("{nextnum}");
     }
}

fn term()   {
    loop{
        println!("Type the nth term of the Fibonacci sequence to see. Note that the first and second term are the same.");
        println!("WARNING: Typing a big number may result in slower display times.");
        println!("Note: Only positive, whole numbers will allow for correct output to be produced.");
        println!("----------------------------------------------------------------------------------------");
        
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("---Failed to read line---");
    
        let  b: u64 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if b > 0    {
            println!("You wanted to see the following term: {b}");
            println!("-----------------------------------------");
    
            let b = b - 1;
         
            let mut gen1 = BigInt::from(1);
            let mut gen2 = BigInt::from(1);
            let mut nextnum = gen2.clone();
            println!("{gen1}");
            println!("{gen2}");
                for _i in 1..b {
                    gen1 = gen2;
                    gen2 = nextnum;
                    nextnum = gen1 + gen2.clone();
                    println!("{nextnum}");
                }
            break
        }
        
        else if b <= 0  {
            println!("Please enter a positive whole number above 0.");
            println!("---------------------------------------------");
            continue;
        }

        else    {
            println!("Please enter a positive whole number above 0.");
            println!("---------------------------------------------");
            continue;
        }

    }
}
