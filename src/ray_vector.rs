use std::ops;
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub elements: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        return Vec3 {
            elements: [e0, e1, e2],
        };
    }

    // coordinates
    pub fn x(&self) -> f32 {
        return self.elements[0];
    }
    pub fn y(&self) -> f32 {
        return self.elements[1];
    }
    pub fn z(&self) -> f32 {
        return self.elements[2];
    }

    // colors
    pub fn r(&self) -> f32 {
        return self.elements[0];
    }
    pub fn g(&self) -> f32 {
        return self.elements[1];
    }
    pub fn b(&self) -> f32 {
        return self.elements[2];
    }

    // Traits?
    pub fn length(&self) -> f32 {
        (self.elements[0] * self.elements[0]
            + self.elements[1] * self.elements[1]
            + self.elements[2] * self.elements[2])
            .sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.elements[0] * self.elements[0]
            + self.elements[1] * self.elements[1]
            + self.elements[2] * self.elements[2]
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        let length_bar = 1.0 / self.length();
        Vec3 {
            elements: [
                self.x() * length_bar,
                self.y() * length_bar,
                self.z() * length_bar,
            ],
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    pub fn to_color_vec(&self) -> [u8; 3] {
        [
            (self.x() * 255.9) as u8,
            (self.y() * 255.9) as u8,
            (self.z() * 255.9) as u8,
        ]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, term: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() + term.x(),
                self.y() + term.y(),
                self.z() + term.z(),
            ],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, term: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() - term.x(),
                self.y() - term.y(),
                self.z() - term.z(),
            ],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, term: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() * term.x(),
                self.y() * term.y(),
                self.z() * term.z(),
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, term: f32) -> Vec3 {
        Vec3 {
            elements: [self.x() * term, self.y() * term, self.z() * term],
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, term: Vec3) -> Vec3 {
        Vec3 {
            elements: [term.x() * self, term.y() * self, term.z() * self],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, term: f32) -> Vec3 {
        Vec3 {
            elements: [self.x() / term, self.y() / term, self.z() / term],
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, term: Vec3) -> Vec3 {
        Vec3 {
            elements: [
                self.x() / term.x(),
                self.y() / term.y(),
                self.z() / term.z(),
            ],
        }
    }
}

impl PartialEq<Vec3> for Vec3 {
    fn eq(&self, term: &Vec3) -> bool {
        self.x() == term.x() && self.y() == term.y() && self.z() == term.z()
    }
}
