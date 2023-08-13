struct Temperature{
    farenhein: f64
}

// imlementacion
impl Temperature{
    // new implementation

    fn freezee()-> Self{
        // return new struct
        Self {
            farenhein : 32.0
        }
    }

    fn show_temp(&self){
        println!("{:?} degress F", self.farenhein);

    }
}


// Exercise
/*Requirements
Use a struct to encapsulate the box characteristics
Use a enum for the box color
Implements functionality on the box struct to create a new box
Implements functionality on the box struct to print the characteristics
*/
enum Colors {
    Blue,
    Yellow,
    Black
}

impl Colors{
    fn print(&self){
       match self{
        Colors::Blue => println!("brow"),
        Colors::Yellow => println!("yellow"),
        Colors::Black => println!("black"),

       }
    }

}

struct Dimensions{
    width: f64,
    height :f64,
    depth : f64

}

impl Dimensions{
    fn print(&self){
        println!("width {:?} ", self.width);
        println!("height {:?} ", self.height);
        println!("dept {:?} ", self.depth);

    }
}

struct ShippingBox{
    color : Colors,
    weight: f64,
    dimensions: Dimensions

}


impl ShippingBox {
    fn newBox(color:Colors, weight:f64, dimensions:Dimensions) -> Self{
        Self{
            color,
            weight,
            dimensions,
        }


    }

    fn print(&self){
        self.color.print();
        println!("wight {:?}", self.weight);
        self.dimensions.print();
    } 
}
fn main() {
    let hot = Temperature{
        farenhein : 99.9
    };

    let cold = Temperature::freezee();
    cold.show_temp();  // return 32 
    

    hot.show_temp(); //return 99

    // dimensions 
    let small_dimensions = Dimensions{
        width:1.0,
        height:1.5,
        depth:3.0,
    };

     

    let small_box = ShippingBox::newBox(Colors::Blue, 5.0, small_dimensions);
    // information box

    small_box.print();

}
