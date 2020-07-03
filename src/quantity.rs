const NUM_DIMENSION_TYPES: usize = 7;

pub struct Quantity {
    magnitude: f64,
    dimensions: [i32; NUM_DIMENSION_TYPES],    
}

impl Quantity {

    /// Create a new Quantity
    /// 
    /// # Arguments
    /// 
    /// * `magnitude` - Raw magnitude of the quantity
    /// * `dimensions` - Array of values that define the "dimensionality" of the quantity
    /// 
    pub fn new(magnitude: f64, dimensions: [i32; NUM_DIMENSION_TYPES]) -> Quantity {
        return Quantity {
            magnitude: magnitude,
            dimensions: dimensions,
        }
    }

    /// Check if two quantities have the same dimensionality
    /// 
    /// # Arguments
    /// 
    /// * `other` - Other quantity to compare this quantity to
    /// 
    /// # Return
    /// 
    /// When the return value is:
    ///   * true: The quantities have the same dimensionality
    ///   * false: The quantities do not have the same dimensionality
    ///
    pub fn same_dimensionality(&self, other: &Quantity) -> bool {
        //let mut return_val: bool = true;
        for dim_idx in 0..(NUM_DIMENSION_TYPES-1) {
            if self.dimensions[dim_idx] != other.dimensions[dim_idx] {
                return false;
            }
        }
        return true;
    }

    /// Add two quantities together
    /// 
    /// # Arguments
    /// 
    /// * `other` - Other quantity to add to this quantity
    /// 
    /// # Return
    /// 
    /// Returns a Result enum with the added quantity if the operation is allowed
    /// 
    pub fn add(&self, other: &Quantity) -> Quantity {
        // Check for user input error: TODO Convert to Result instead of panic!
        if !self.same_dimensionality(other) {
            panic!("Cannot add two quantities with unlike types!");
        };
        return Quantity {
            magnitude: self.magnitude + other.magnitude,
            dimensions: self.dimensions
        }
    }

    /// Check if two quantities are equal to each other
    /// 
    /// # Arguments
    /// 
    /// * `other` - Other quantity to check for equality with this quantity
    /// 
    /// # Return
    /// 
    /// * true - quantities are equal
    /// * false quantities are not equal
    /// 
    pub fn eq(&self, other: &Quantity) -> bool {
        if !self.same_dimensionality(other) || (self.magnitude != other.magnitude) {
            return false;
        };
        return true;
    }
}
