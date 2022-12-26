mod tensor;
use tensor::Tensor;

fn main() {
    let data:Vec<f64> = vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
    let tensor3:Tensor= Tensor::create_tensor(data, &[3,2]);

    tensor3.show();

}
