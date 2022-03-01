fn main(){
    // let a : u8 = 12;
    // println!("{}",a);
    // let mut toplam = 0;
    // for i in 5..2002{
    //     if i % 3 == 0 || i%5 == 0{
    //             toplam += i;
    //     } 
    // }

    // println!("Toplam={}", toplam)

    let mut sayi = 20;
    
    loop{
        let mut a = true;
        for i in 2..17{
            if sayi % i == 0{
                continue;
            }
            a = false;
            break;
        }
        if a{
            break;
        }
        sayi += 16 ;
    }

    println!("Toplam={}", sayi)

}