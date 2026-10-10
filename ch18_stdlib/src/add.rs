use std::fmt;
use std::ops::Add; // Add를 먼저 추가합니다.

// PartialEq는 아마도 여기서 가장 중요한 부분일 것입니다.
// 우리는 숫자를 비교하고 싶습니다.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // 다음을 '연관 타입', 즉, '함께 사용되는 타입'이라고 합니다.
    // 이 경우에는 다른 Point에 불과합니다.

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32, // 경제 규모를 나타내는 지표
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, other.name), // 이름을 함께 담고,
            population: self.population + other.population,    // 인구를 합산하며,
            gdp: self.gdp + other.gdp,                         // GDP도 합산합니다.
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // 그러면 {}만 사용해 모두 출력할 수 있습니다.
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

fn main() {
    let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("Vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("Micronesia", 104_468, 367_000_000);

    // Country에 이름에 문자열 대신 &str을 지정할 수도 있었습니다.
    // 하지만 모든 곳에 수명을 사용해야 하고
    // 간단한 예제에서는 너무 많은 양이 될 것입니다.
    // println!을 호출할 때 그냥 복제하는 편이 낫습니다.
    println!("{}", nauru.clone());
    println!("{}", nauru.clone() + vanuatu.clone());
    println!("{}", nauru + vanuatu + micronesia);
}
