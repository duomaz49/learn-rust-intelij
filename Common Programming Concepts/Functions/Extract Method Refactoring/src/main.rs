fn main() {
    print_ten_years_ago(1993);

    print_ten_years_ago(2031);
}

fn print_ten_years_ago(year: i32) {
    println!("{}: ten years ago was {}", year, year - 10);
}