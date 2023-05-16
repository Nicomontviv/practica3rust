struct Rectangulo{
    longitud: f64,
    ancho:f64,
}

impl Rectangulo{
    fn new(longitud:f64, ancho:f64) -> Rectangulo{
        Rectangulo{
            longitud,
            ancho,
        }
    }
    fn calcular_area(&self) -> f64{
        let area:f64 = self.longitud * self.ancho;
        return area
    }
    fn calcular_perimetro(&self) -> f64{
        let perimetro:f64 = (self.longitud * 2.0) + (self.ancho * 2.0);
        return perimetro
    }
    fn es_cuadrado(&self) -> bool{
        let mut cuadrado:bool = false;
        if(self.longitud == self.ancho){
            cuadrado = true;
        }
        return cuadrado
    }
    fn imprimir(&self){
        println!("La longitud del rectangulo es {} y el ancho es {}", self.longitud, self.ancho);
    }
}
fn main(){
    let rectangulo1 = Rectangulo::new(33.4 , 54.8);
    rectangulo1.imprimir();
    println!("El area del rectangulo es: {}",rectangulo1.calcular_area());
    println!("El perimetro del rectangulo es: {}", rectangulo1.calcular_perimetro());
}