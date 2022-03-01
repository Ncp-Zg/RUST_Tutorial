fn main(){
    let sayi = 19;
    println!("{:?}",asal(sayi));

    let mut en_buyuk = 888*888;
    loop{
        if palindrom(en_buyuk) && bolum(en_buyuk){
            println!("en büyük palindrom = {}", en_buyuk);
            break;
        }
        en_buyuk -= 1;
    }
}

fn asal(sayi: u64) -> bool{
    for i in (3..sayi/2).step_by(2){
        if sayi % i == 0 || sayi % 2 == 0{
            return false;
        }
    }
    true
}

fn palindrom(sayi: u32) -> bool{
    if sayi.to_string() == sayi.to_string().chars().rev().collect::<String>(){
        return true;
    }
    false
}

fn bolum(sayi : u32) -> bool{
    for i in 100..1000{
       if sayi % i == 0 {
            if sayi/i < 999 && sayi/i > 100 {
                return true;
            }        
        }
    }
    false
}