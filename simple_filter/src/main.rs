//3
struct FilterCondition {
    value: i32,
}
//4
impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}
//5
fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_collection = Vec::new();

    for &item in collection {
        if condition.is_match(&item) {
            filtered_collection.push(item);
        }
    }

    filtered_collection
}
fn main() {
    //6
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    //6
    let filter = FilterCondition { value: 5 };

    //7
    let filtered_numbers = custom_filter(&numbers, &filter);

    //7
    println!("Filtered numbers: {:?}", filtered_numbers);
}
