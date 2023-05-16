#[derive(Debug)]
enum Tipo{
    ISOSELES,
    EQUILATERO,
    ESCALENO, 
    NODEFINIDO,
}
struct Triangulo {
    lado1:f64,
    lado2:f64,
    lado3:f64,
    tipo:Tipo,
}
impl Triangulo{
    fn new(lado1:f64, lado2:f64, lado3:f64) ->Triangulo{
        Triangulo{
             lado1,
             lado2,
             lado3,
             tipo:Tipo::NODEFINIDO,
        }
    }
    fn determinar_tipo(&mut self) -> Tipo{
        if self.lado1 == self.lado2 || self.lado2 == self.lado3 || self.lado3 == self.lado1{
            self.tipo = Tipo::ISOSELES;        
            return Tipo::ISOSELES  
                }else if self.lado1 == self.lado2 && self.lado2 == self.lado3{
                    self.tipo = Tipo::EQUILATERO;
                    return Tipo::EQUILATERO
                }else{
                    self.tipo = Tipo::ESCALENO;
                    return Tipo::ESCALENO
                }    
      }
    fn calcular_area(&self) -> f64{
        let area:f64 = (self.lado1 * self.lado2)/2.0;
        return area
    }
    fn calcular_perimetro(&self) -> f64{
        let perimetro:f64 = self.lado1 + self.lado2 + self.lado3;
        return perimetro
    }
}
fn main() {
    let mut triangulo1 = Triangulo::new(23.0,23.0,19.5);
    println!("El triangulo es de tipo {:?}", triangulo1.determinar_tipo());
    println!("El area del trtiangulo es {}", triangulo1.calcular_area());
    println!("El perimetro de un triangulo es {}", triangulo1.calcular_perimetro());
}