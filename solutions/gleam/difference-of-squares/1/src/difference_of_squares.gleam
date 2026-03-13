import gleam/list
import gleam/int

pub fn square_of_sum(n: Int) -> Int {
  let sum = list.fold(list.range(1, n), 0, fn(acc, x) { acc + x })
  sum * sum
}
pub fn sum_of_squares(n: Int) -> Int {
  list.range(1, n)
  |> list.map(fn(x) { x*x })
  |> list.fold(0, fn(acc, x) { acc + x })
}

pub fn difference(n: Int) -> Int {
  int.absolute_value(square_of_sum(n) - sum_of_squares(n))
}
