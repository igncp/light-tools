fn get_accrued_interest(interest_rate: f64, nominal_amount: f64, days: usize) -> f64 {
  interest_rate * nominal_amount * (days as f64) / 365.0
}

// 2.1
pub fn make_accrued_interest_calculations() {
  let days = 3 + 31 + 30 + 15;
  let accrued_interest = get_accrued_interest(0.15, 100.0, days);

  println!("accrued_interest: {:.2}", accrued_interest);
}
