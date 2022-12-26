use ndarray::{Array, IxDyn};

pub struct Tensor {
    data: Array<f64, IxDyn>,
}

impl Tensor {
    pub fn create_tensor(vec:Vec<f64>, shape:&[usize]) -> Tensor{
        Tensor { data: Array::from_shape_vec(IxDyn(shape), vec).unwrap() }
    }

    pub fn show(self:&Self){
        println!("{:?}", self.data);

    }
}

