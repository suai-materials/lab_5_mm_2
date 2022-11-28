use rand::random;

/**
В трехканальную СМО с отказами поступает пуассоновский поток заявок. Время между моментами поступления двух последовательных заявок распределено по закону f() = 8e-8; время обслуживания заявок = 1 мин.
Найти методом Монте-Карло математическое ожидание а числа обслуженных заявок за время Т=8 мин.
 */

fn main() {
    let mut r: f64;
    let mut T: f64 = 0.;
    let mut t_i: f64;
    let mut i = 0;
    let interval = 1.;
    let mut channels: [f64; 3] = [interval, 0., 0.];
    let mut added = 0;
    let mut not_added = 0;
    while T < 8. {
        i += 1;
        r = random();
        t_i = -1. / 8. * r.ln();
        let mut is_added = false;
        for (i, el) in channels.iter().enumerate() {
            if *el <= T {
                channels[i] = t_i + interval + T;
                is_added = true;
                added += 1;
                break;
            }
        }
        if !is_added {
            not_added += 1;
        }
        T += t_i;
        println!("Номер заявки: {i} T = {}", T);
        for channel in channels {
            print!("{channel}\t");
        }
        println!();
    }
    println!("Обслуженных заявок: {added}  Необслуженных заявок: {not_added}")
}
