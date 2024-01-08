use std::error::Error;

mod example {
    #[derive(Debug, Default)]
    pub struct CarBuilder {
        name: Option<String>,
        color: Option<String>,
        weight: Option<f32>,
        max_speed: Option<u32>,
    }

    impl CarBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn name(mut self, name: &str) -> Self {
            self.name = Some(name.to_string());
            self
        }

        pub fn color(mut self, color: &str) -> Self {
            self.color = Some(color.to_string());
            self
        }

        pub fn weight(mut self, weight: f32) -> Self {
            self.weight = Some(weight);
            self
        }

        pub fn max_speed(mut self, max_speed: u32) -> Self {
            self.max_speed = Some(max_speed);
            self
        }

        pub fn build(self) -> Result<Car, &'static str> {
            Ok(Car {
                name: self.name.ok_or("name is required")?,
                color: self.color.ok_or("color is required")?,
                weight: self.weight.ok_or("weight is required")?,
                max_speed: self.max_speed.ok_or("max_speed is required")?,
            })
        }
    }

    #[non_exhaustive] // prevent users from constructing the struct directly
    #[derive(Debug)]
    pub struct Car {
        name: String,
        color: String,
        weight: f32,
        max_speed: u32,
    }

    impl Car {
        pub fn new() -> CarBuilder {
            CarBuilder::new()
        }
    }
}

pub fn builder() -> Result<(), Box<dyn Error>> {
    use example::Car;

    let car = Car::new()
        .name("Ferrari")
        .color("Red")
        .weight(1000.0)
        .max_speed(300)
        .build()?;

    println!("{:?}", car);

    Ok(())
}
