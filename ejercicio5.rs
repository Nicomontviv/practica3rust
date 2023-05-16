#[derive(Debug)]
struct Producto{
    nombre:String,
    precio_bruto:f64,
    num_id:i32,
}
impl Producto{
    fn new(un_nombre:String, un_precio_bruto:f64, un_num_id:i32) -> Producto{
         Producto{
            nombre:un_nombre,
            precio_bruto:un_precio_bruto,
            num_id:un_num_id,
         }
    }

    fn calcular_impuestos(&self,porcentaje_de_impuestos:f64) -> f64{
              let impuesto:f64 = (porcentaje_de_impuestos * self.precio_bruto)/100.0;
              return impuesto
    }
    fn aplicar_descuento(&self,porcentaje_de_descuento:f64) -> f64{
        let descuento:f64 = (porcentaje_de_descuento * self.precio_bruto)/100.0;
        return descuento
    }
    fn calcular_precio_total(&self, porcentaje_de_impuestos:Option<f64>, porcentaje_de_descuento:Option<f64>) ->f64{
        let mut precio_total:f64 = 0.0;
        match porcentaje_de_impuestos{
            Some(a)=> precio_total = self.precio_bruto + self.calcular_impuestos(a),
            None => precio_total = self.precio_bruto,
        }
        match porcentaje_de_descuento{
            Some(a)=> precio_total = precio_total - self.aplicar_descuento(a),
            None => precio_total = precio_total,
        }
        return precio_total
    }
}
fn main() {
    let producto1 = Producto::new("Remera".to_string(), 40.6, 17);
    println!("{:?}",producto1);
    println!("El precio final es {}" ,producto1.calcular_precio_total(Some(20.0),Some(3.0)));
}