mod accrued_interest;
mod discount_payback_period;

use accrued_interest::make_accrued_interest_calculations;
use discount_payback_period::make_discount_payback_period_calculations;

#[derive(Debug)]
struct PeriodicCompoundingOpts {
  principal_sum: f64,
  nominal_annual_interest_rate: f64,
  compounding_frequency: f64,
  length_of_time: f64,
}

/*
 * https://en.wikipedia.org/wiki/Compound_interest
 * P' = P * (1 + (r / n)) ^ (n * t)
 *
 * P ($) is the original principal sum
 * P' ($) is the new principal sum
 * r (1 / time unit) is the nominal annual interest rate
 * n (1 / time unit) is the compounding frequency
 * t (time unit) is the overall length of time the interest is applied (expressed using the same time units as r, usually years).
*/

fn get_new_principal_sum(opts: &PeriodicCompoundingOpts) -> f64 {
  let mut result: f64 = 1.0 + opts.nominal_annual_interest_rate / opts.compounding_frequency;
  result = result.powf(opts.compounding_frequency * opts.length_of_time);
  result = result * opts.principal_sum;

  result
}

// This calculates P instead of P'
fn get_present_value(opts: &PeriodicCompoundingOpts) -> f64 {
  let mut result: f64 = 1.0 + opts.nominal_annual_interest_rate / opts.compounding_frequency;
  result = result.powf(opts.compounding_frequency * opts.length_of_time);
  result = opts.principal_sum / result;

  result
}

fn get_total_compound_interest(principal_sum: f64, new_principal_sum: f64) -> f64 {
  new_principal_sum - principal_sum
}

#[derive(Debug)]
struct EffectiveInterestOpts {
  times_in_year: f64,
  value: f64,
}

/*
 *
 * (1 + j) ^ a = (1 + k) ^ b
 *
 * j: interest for 'a' times in a year
 * k: interest for 'b' times in a year
 *
 */
fn get_effective_interest_for_other_time(
  opts: &EffectiveInterestOpts,
  to_times_in_year: f64,
) -> f64 {
  let mut computed_value = opts.value + 1.0;
  computed_value = computed_value.powf(opts.times_in_year / to_times_in_year);
  computed_value = computed_value - 1.0;

  computed_value
}

fn get_force_of_interest(annual_rate: f64) -> f64 {
  let mut return_value = annual_rate + 1.0;
  return_value = return_value.ln();

  return_value
}

fn make_interest_conversions_calculations() {
  // 1.4
  let opts_year_interest = EffectiveInterestOpts {
    times_in_year: 1.0,
    value: 0.07,
  };
  let monthly_effective_interest = get_effective_interest_for_other_time(&opts_year_interest, 12.0);

  println!("{:?}", opts_year_interest);
  println!("{:.5}", monthly_effective_interest);

  println!("");

  // 1.5
  let orig_interest = 0.06;
  let force_of_interest = get_force_of_interest(0.06);

  println!("orig_interest: {:.2}%", orig_interest * 100.0);
  println!("get_force_of_interest: {:.2}%", force_of_interest * 100.0);
}

fn make_compound_interest_calculations() {
  // Example 1 in Wikipedia
  let opts = PeriodicCompoundingOpts {
    principal_sum: 1500.0,
    nominal_annual_interest_rate: 0.043,
    compounding_frequency: 4.0,
    length_of_time: 6.0,
  };
  let new_principal_sum = get_new_principal_sum(&opts);
  let total_compound_interest = get_total_compound_interest(opts.principal_sum, new_principal_sum);

  println!("opts: {:?}", opts);
  println!("new_principal_sum: {:.1}", new_principal_sum);
  println!("total_compound_interest: {:.1}", total_compound_interest);

  println!("");

  // Example 2 in Wikipedia
  let opts2 = PeriodicCompoundingOpts {
    principal_sum: 1500.0,
    nominal_annual_interest_rate: 0.043,
    compounding_frequency: 0.5,
    length_of_time: 6.0,
  };
  let new_principal_sum2 = get_new_principal_sum(&opts2);
  let total_compound_interest2 =
    get_total_compound_interest(opts2.principal_sum, new_principal_sum2);

  println!("opts: {:?}", opts2);
  println!("new_principal_sum: {:.1}", new_principal_sum2);
  println!("total_compound_interest: {:.1}", total_compound_interest2);

  println!("");

  // 1.6
  let opts = PeriodicCompoundingOpts {
    principal_sum: 100.0,
    nominal_annual_interest_rate: 0.07,
    compounding_frequency: 1.0,
    length_of_time: 8.0,
  };
  let present_value = get_present_value(&opts);

  println!("opts: {:?}", opts);
  println!("present_value: {:.1}", present_value);
}

fn main() {
  make_compound_interest_calculations();
  println!("");
  make_interest_conversions_calculations();
  println!("");
  make_discount_payback_period_calculations();
  println!("");
  make_accrued_interest_calculations();
}
