/// Constructs a new 'Vec<f32>' where each element contains calculated
/// typical price.  Formula:  Typical Price (TP) = (High + Low + Close) / 3
///
/// # Examples
///
/// '''
/// use rust_stock_technicals::indicators::typical_price
///
/// let typical_prices = typical_price::execute(&high,&low,&close);
/// '''
pub fn execute(high: &Vec<f32>, low: &Vec<f32>, close: &Vec<f32>) -> Vec<f32> {

	if  high.len() != low.len() || high.len() != close.len() {
		panic!("Mismatching array size");
	}
	
	let mut tp = vec![];
	
	for i in 0..high.len() {
		let val : f32 = (high[i] + low[i] + close[i]) / 3.0;
        tp.push( val );
    }
	
	tp
}

#[cfg(test)]
mod test {

	use super::*;

    #[test]
    fn typical_price_test() {

    	let high  = vec![24.20, 24.07, 24.04, 23.87, 23.67, 23.59];
    	let low   = vec![23.85, 23.72, 23.64, 23.37, 23.46, 23.18];
    	let close = vec![23.89, 23.95, 23.67, 23.78, 23.50, 23.32];
    		
    	let typical_prices = execute(&high,&low,&close);
    	
        assert_eq!( (format!("{:.2}",typical_prices[0])).parse::<f32>().unwrap(),23.98);
        assert_eq!( (format!("{:.2}",typical_prices[1])).parse::<f32>().unwrap(),23.91);
        assert_eq!( (format!("{:.2}",typical_prices[2])).parse::<f32>().unwrap(),23.78);
        assert_eq!( (format!("{:.2}",typical_prices[3])).parse::<f32>().unwrap(),23.67);
        assert_eq!( (format!("{:.2}",typical_prices[4])).parse::<f32>().unwrap(),23.54);
        assert_eq!( (format!("{:.2}",typical_prices[5])).parse::<f32>().unwrap(),23.36);
    }
}