fn main(){
    let mut numbers = vec![1,2];
    loop{
        let ln = numbers.len();
        let yeni_sayi = numbers[ln-1]+numbers[ln-2];
        if yeni_sayi < 2000000{
            numbers.push(yeni_sayi);
            continue;
        }
        break;
    }

    let mut toplam = 0;
    for number in numbers{
        if number % 2 == 1{
            toplam += number;
        }
    }

    println!("{:?}", toplam)
}