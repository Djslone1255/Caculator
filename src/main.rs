/*does RUST just hate encapsulation, because it feels like it does
struct Equation {
    n1: f32,
    n2: f32,
    sign: i8,

    result: f32
}
impl Equation {

    fn getResult(&self) {
        if self.sign == 0{
            //self.result = add(self.n1, self.n2);
        }
    }
}*/




fn main() {
    let numbers = [22, 34, 50, 29, 70, 61, 24, 6];
    let actions = ['+', '-', '*', '/'];
    let mut x = 0;
    while x < 8 {
        let n1 = numbers[x];
        let n2 = numbers[x+1];
        let sign = actions[x/2];

        let result = do_math(n1 as f32, n2 as f32, sign);

        println!("{} {} {} = {}",n1, sign, n2, result);
        x += 2;
    }
    
    //let mut line = String::new();
    //let n1 = std::io::stdin().read_line(&mut line).unwrap();
    //let n2 = std::io::stdin().read_line(&mut line).unwrap();
    //n1.trim();

}


fn do_math(n1: f32, n2: f32, sign: char) -> f32{
    //why is it needed to be a return to do anything with a conditonal
    if sign == '+' {
        add(n1, n2)
    } else if sign == '-'{
        subtract(n1, n2)
    } else if sign == '*'{
        multiply(n1, n2)
    } else if sign == '/'{
        divide(n1, n2)
    } else {
        n1 // this is dumb
    }
        
}


/*what is RUST user input, why is it so complicated,
 the more I work the more this language looks stupid.
 easier to use than c++ is a bold faced lie
fn get_input() -> f32 {
    //From Stack overflow
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<f32>() {
        Ok(i) => i
        Err(..) => println!("this was not an integer: {}", trimmed),
    }; 
}*/

fn add(n1: f32, n2: f32) -> f32 {
    n1 + n2
}

fn multiply(n1: f32, n2: f32) -> f32 {
    n1 * n2
}

fn divide(n1: f32, n2: f32) -> f32 {
    n1 / n2
}

fn subtract(n1: f32, n2: f32) -> f32 {
    n1 - n2
}

