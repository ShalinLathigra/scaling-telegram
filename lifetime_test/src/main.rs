fn main() {
    let longest = String::from("longest string you've ever seen in your whole dang life");
    let shortest = String::from("shrtst");

    // Demonstrate proper usage of longest
    let result = longest_full(longest.as_str(), shortest.as_str());
    println!("Here is the longest string: {}", result);

    let result;
    {
        // this is still valid because both options are in scope, result is used inside scope
        let middlest = String::from("I'm the middlest string");
        result = longest_full(shortest.as_str(), middlest.as_str());
        println!("Now this is the longest string: {}", result)
    }
    // Improper use of longest (dependent on variable from an inner scope)
    // printing this out fails as the result is understood to last as long as the shortest lived input
    // println!("Now this is the longest string: {}", result)

    // let result = longest_full(longest.as_str(), middlest.as_str());

    // Can also define functions using partial strings

    let result;
    {
        let middlester = String::from("I'm a more middler string");
        result = longest_partial(longest.as_str(), middlester.as_str());
    }
    // This works because result has a lifetime matching longest
    // If the inputs were swapped this would break
    println!("longest partial thinks this is the one: {}", result);

    // fourth, a dangling reference

    // let result = longest_dangle(longest.as_str(), shortest.as_str());
    // println!("This is a dangling ref: {}", result);
}

fn longest_full<'a>(x: &'a str, y: &'a str) -> &'a str {
    // compare length of the two strings
    // return the longest string
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn longest_partial<'a>(x: &'a str, _y: &str) -> &'a str {
    // just return the first one
    x
}

// Can't even use this function because it is attempting to borrow data stored inside the function
// fn longest_dangle<'a>(_x: &str, _y: &str) -> &'a str {
//     let result = String::from("longest string you've ever seen");
//     result.as_str()
// }
