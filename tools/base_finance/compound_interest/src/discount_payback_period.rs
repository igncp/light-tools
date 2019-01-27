#[derive(Debug)]
struct GetDiscountPaybackPeriodOpts {
  cost_of_capital_per_period: f64,
  get_cash_inflow_for_n: fn(usize) -> f64,
  get_cash_outflow_for_n: fn(usize) -> f64,
}

fn get_discount_cash_flow(net_cash_flow: f64, cost_of_capital: f64, period: usize) -> f64 {
  let mut discount_factor = 1.0 + cost_of_capital;
  discount_factor = discount_factor.powf((period as f64) * -1.0);

  net_cash_flow * discount_factor
}

fn get_discount_payback_period_with_npv(opts: &GetDiscountPaybackPeriodOpts) -> (usize, f64) {
  let mut acc_dcf: f64 = 0.0;
  let mut discount_payback_period = 0;

  for n in 0..10 {
    discount_payback_period = n;
    let mut net_cash_flow = -(opts.get_cash_outflow_for_n)(n);
    net_cash_flow += (opts.get_cash_inflow_for_n)(n);

    let dcf = get_discount_cash_flow(net_cash_flow, opts.cost_of_capital_per_period, n);

    acc_dcf += dcf;

    if acc_dcf >= 0.0 {
      break;
    }
  }

  (discount_payback_period, acc_dcf)
}

pub fn make_discount_payback_period_calculations() {
  fn get_cash_outflow_for_n(n: usize) -> f64 {
    let result = match n {
      0 => 5000,
      1 => 50,
      2 => 50,
      3 => 100,
      4 => 100,
      5 => 150,
      6 => 200,
      _ => 0,
    };
    result as f64
  }

  fn get_cash_inflow_for_n(n: usize) -> f64 {
    let result = match n {
      0 => 0,
      1 => 1000,
      2 => 1500,
      3 => 2000,
      4 => 1500,
      5 => 1000,
      6 => 500 + 1000,
      _ => 500,
    };
    result as f64
  }

  let opts = GetDiscountPaybackPeriodOpts {
    cost_of_capital_per_period: 0.1,
    get_cash_inflow_for_n: get_cash_inflow_for_n,
    get_cash_outflow_for_n: get_cash_outflow_for_n,
  };

  let (discount_payback_period, acc_dcf) = get_discount_payback_period_with_npv(&opts);

  println!("{:?}", opts);
  println!("discount_payback_period: {}", discount_payback_period);
  println!("acc_dcf: {:.2}", acc_dcf);
}
