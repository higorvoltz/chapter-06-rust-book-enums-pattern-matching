enum IpAddressKind {
  V4,
  V6,
}

enum IpAddressKindV2 {
  V4(i8, i8, i8, i8),
  V6(String),
}

struct IpAddress {
  kind: IpAddressKind,
  address: String,
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

enum Option<T> {
  None,
  Some(T),
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

fn main() {

  let _localhost = IpAddress {
    kind: IpAddressKind::V4,
    address: String::from("127.0.0.1"),
  };

  let _locahost_v2 = IpAddressKindV2::V4(127, 0, 0, 1);


  // let none = plus_one(None);
  // |              -------- ^^^^ expected enum `Option`, found enum `std::option::Option`



}