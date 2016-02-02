
// Typical Price (TP) = (High + Low + Close)/3

pub fn execute(high: &Vec<f64>, low: &Vec<f64>, close: &Vec<f64>) -> Vec<f64> {

	if  high.len() != low.len() || high.len() != close.len() {
		panic!("Mismatching array size");
	}
	
	let mut tp = vec![];
	
	for i in 0..high.len() {
		let val : f64 = (high[i] + low[i] + close[i]) / 3.0;
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
    	
    	//println!("Value ================> {}",typical_prices[0]);
    	
        assert!(typical_prices[0] == 23.98);
     
    }
}