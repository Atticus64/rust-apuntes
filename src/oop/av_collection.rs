struct AvCollection {
    average: f64,
    list: Vec<i32>,
}

impl AvCollection {
    /// Add a item in the list
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// Remove a item in the list
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    /// Get the value average of the list
    pub fn average(&self) -> f64 {
        self.average
    }

    /// Update the average value
    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();

        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut data = AvCollection {
        list: Vec::new(),
        average: 0.0,
    };

    data.add(10);
    data.add(5);
    data.add(4);
    data.add(2);
    data.add(29);

    data.remove();

    println!("The average is {}", data.average());
}
