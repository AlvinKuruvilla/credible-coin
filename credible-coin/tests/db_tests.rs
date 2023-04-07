#[cfg(test)]
mod tests{
	use credible_coin::utils::db_func::*;
	use std::path::Path;
	
	#[test]
	pub fn create_db_test(){
		createDB(); ///FIX
		assert!(Path::new("test.csv").try_exists().expect("Can't find the file"));
	}



}