struct Score{
    score: i32
}

fn main() {
    // example vectors

    let my_numbers = vec![1,2,3];

    let mut my_numbers =  Vec::new();

    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.pop();
    my_numbers.len(); // this is 1


    // array score

    let my_scores = vec![
        Score{
            score:80
        },

        Score{
            score:55
        },
        Score{
            score:63
        },
        Score{
            score:99
        }
    ];

    for test in my_scores{
        println!("score = {:?}", test.score);
    }

    // exercise 
    /*
    Requirements
    pRINT 10,20 "THIRTHY" and 40 in a lopp
    Print the total numbers of elements in a vector
    Notes
    Use a Vector to store 4 numbers
    Interate through thje vector using a for in lopp
    Determinate wheter to print the number or print "thirty" inside
    Use the .len function to print the number of elements in a vector */

    let my_numbersTwo = vec![10,20,30,40];

    for num in &my_numbersTwo{
        match num{
            30 => println!("thirthy"),
           _ => println!("{:?}", num)
        }

       
    }

    println!("The total vector long {:?}", my_numbersTwo.len());
}
