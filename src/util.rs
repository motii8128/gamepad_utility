use crate::types::{Axis, Buttons, ControllerData};

impl Axis {
    pub fn new()->Self
    {
        Axis { x: 0.0, y: 0.0 }
    }
}
impl Buttons {
    pub fn new()->Self
    {
        Buttons { circle: 0, triangle: 0, cube: 0, cross: 0, r1: 0, r2: 0, l1: 0, l2: 0 }
    }
}

impl ControllerData {
    pub fn new()->Self
    {
        ControllerData { left_stick: Axis::new(), right_stick: Axis::new(), dpad: Axis::new(), btns: Buttons::new() }
    }
}

pub fn bool_to_f32(b : bool)->f32
{
    if b
    {
        1.0
    }
    else 
    {
        0.0
    }
}

pub fn bool_to_i8(b: bool)->i8
{
    if b
    {
        1
    }
    else {
        0
    }
}