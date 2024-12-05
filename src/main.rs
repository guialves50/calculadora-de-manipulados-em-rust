use std::io;
use clearscreen;
use colour::*;

fn main() {
    dark_red_ln_bold!("CALCULADORA DE MANIPULADOS EM RUST\n\n");

    let mut valor_custo = String::new();
    let mut count_loop = 0u32;
    let mut loop_breaker = String::new();
    let mut counter: i16 = 0;
    let mut switch = String::new();

    println!("Quantos manipulados deseja precificar?");

    io::stdin()
        .read_line(&mut loop_breaker)
        .expect("Cannot read this line");

    let loop_breaker: u32 = loop_breaker
        .trim()
        .parse()
        .expect("Quantos manipulados deseja precificar");

    loop {
        count_loop += 1;
        counter += 1;

        valor_custo.clear();

        green_ln_bold!("\n{} | {}", counter, loop_breaker);
        println!("Qual o preço de custo?");

        io::stdin()
            .read_line(&mut valor_custo)
            .expect("Cannot read this line");

        let valor_custo: f32 = match valor_custo.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Entrada inválida para o preço de custo. Tente novamente.");
                counter -= 1;
                continue;
            }
        };

        let first_multiplication = valor_custo * 0.25;
        let first_count = valor_custo - first_multiplication;
        let count = first_count + first_count * 0.50;

        println!("O valor final é: {:.2}\n", count);

        if count_loop == loop_breaker {
            dark_red_ln_bold!("Foram precificados {} manipulados!", loop_breaker);

            println!("Digite 1 para sair ou digite 2 para precificar mais manipulados");
        
            io::stdin()
            .read_line(&mut switch)
            .expect("Cannot read this line");
        
            let switch: u16 = switch.trim().parse().expect("Digite:");
        
            if switch == 1 {
                break;
            }
            if switch == 2 {
                clearscreen::clear().expect("failed to clear screen");
                main();
            }
        }
    }
}
