enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}


struct SeaCreature {
    //animal_type: String,
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}



fn main() {
    println!("Hello, world!");
    let mut aa:i32=1;
    aa=2;
    println!("a={}",aa);

    // Integers: 8, 16, 32, 64, 128 bits
    // Default is 32 bits
    let a:i128 = -42; //
    let b:u32 = 42; // 32 bits

    // Floating point numbers, default is 64 bits
    let c= 2.0; // default 64 bits
    let d:f32 =3.0;

    // Boolean values
    let e:bool = true;
    let f:bool = false;

  

    // Character type, fixed 4 bytes (32 bits)
    // Unicode encoding
    let g:char = 'A';
    let h:char = '中';
    let r:char = '🚀';
    let j:char = 'ω';

    // String type, variable length, mutable
    // UTF-8 encoding
    let s:String = String::from("Hello");
    let s2:String = String::from("World");
    let s3:String = s + &s2;
    println!("s3={}",s3);

    // Tuple: fixed length, immutable
    // Tuples can contain elements of different types
    // Compound type
    let k: (i32, f64, u8) = (500, 6.4, 1);
    let k1:i32 = k.0;
    let k2:f64 = k.1;
    let k3:u8 = k.2;

  
    // Array: fixed length, immutable
    let m: [i32; 5] = [1, 2, 3, 4, 5];

    // Function
    println!("{}",function_study(1));


    // Ownership
    let s1 = String::from("hello");
    //let s2 = s1;
    //println!("s1={}",s1); // s1 is no longer valid

    let s2: String = s1.clone(); 
    println!("s1={}",s1);
    println!("s2={}",s2);

    let ferris: SeaCreature = SeaCreature {
        //species: String::from("crab"),
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah: SeaCreature = SeaCreature {
        //species: String::from("octopus"),
        species: Species::Octopus,
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("none"),
    };

    // println!(
    //     "{} is a {}. They have {} arms and {} legs, and a {} weapon",
    //     ferris.name, ferris.species, ferris.arms, ferris.legs, ferris.weapon
    // );
    // println!(
    //     "{} is a {}. They have {} arms and {} legs, and a {} weapon",
    //     sarah.name, sarah.species, sarah.arms, sarah.legs, sarah.weapon
    // );

    match ferris.species {
        Species::Crab => println!("{} is a crab", ferris.name),
        Species::Octopus => println!("{} is a octopus", ferris.name),
        Species::Fish => println!("{} is a fish", ferris.name),
        Species::Clam => println!("{} is a clam", ferris.name),
    }



}

fn function_study(x:i32) -> i32 {
     x+1
}
