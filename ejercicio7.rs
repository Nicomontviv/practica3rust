use std::convert::TryInto;
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Color{
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO,
}
struct ConcesionarioAuto{
    nombre:String,
    direccion:String,
    capacidad_maxima:i32,
    capacidad_usada:i32,
    autos:Vec<Auto>,
}
struct Auto{
    marca:String,
    modelo:String,
    anio:i32,
    precio_bruto:f64,
    color: Color,
}
impl ConcesionarioAuto{
    fn new(unNombre:String, unaDireccion:String, unaCapacidadMaxima:i32) -> ConcesionarioAuto{
        ConcesionarioAuto{
                nombre:unNombre,
                direccion:unaDireccion,
                capacidad_maxima:unaCapacidadMaxima,
                capacidad_usada:0,
                autos: Vec::new(),
        }
    }
    fn agregar_auto(&mut self, auto:Auto) ->bool{
            let agregado:bool;
            if self.capacidad_usada < self.capacidad_maxima {
             self.autos.push( auto);
             agregado = true;
             self.capacidad_usada = self.capacidad_usada + 1;
            }else{
                agregado = false;
            }
            return agregado
    }
    fn eliminar_auto(&mut self) -> bool{
        if self.capacidad_usada > 0 {
            self.autos.pop();
            self.capacidad_usada = self.capacidad_usada - 1;
            return true
        }else{
            return false
        }
    }
    fn buscar_auto(&self ,auto:Auto) -> Option<&Auto>{
         let mut i:i32 = 0;
         while !self.autos[i as usize].eq(&auto) && i < (self.capacidad_maxima as usize).try_into().unwrap(){
            i = i + 1;
         }
         if self.autos[i as usize].eq(&auto){
            let encontrado = &self.autos[i as usize];
            return Some(encontrado)
         } else{ return None}
    }

}
impl Auto{
    fn new(marca:String, modelo:String, anio:i32, precio_bruto:f64, color:Color) ->Auto{
        Auto{
            marca, 
            modelo,
            anio,
            precio_bruto,
            color,
        }
    }
    fn calcular_precio(&self) ->f64{
        let mut precio_final = self.precio_bruto;
        if std::mem::discriminant(&self.color) == std::mem::discriminant(&Color::AZUL) || std::mem::discriminant(&self.color) == std::mem::discriminant(&Color::ROJO) || std::mem::discriminant(&self.color) == std::mem::discriminant(&Color::AMARILLO) {
            precio_final = precio_final + ((self.precio_bruto * 25.0)/100.0);
        }else{
             precio_final = precio_final - ((self.precio_bruto * 10.0)/100.0);
        }
        if self.marca.eq("BMW"){
            precio_final = precio_final + ((precio_final * 15.0)/100.0);
        }
        if self.anio < 2000 {
            precio_final = precio_final - ((precio_final * 5.0)/100.0);
        }
        return precio_final
    }
}
impl PartialEq for Auto {
    fn eq(&self, other: &Self) -> bool {
        self.marca == other.marca && self.modelo == other.modelo && self.anio == other.anio && std::mem::discriminant(&self.color) == std::mem::discriminant(&other.color) && self.precio_bruto == other.precio_bruto
    }
}
impl Clone for Auto {
        fn clone(&self) -> Auto {
            Auto {
                marca: self.marca.clone(),
                modelo: self.modelo.clone(),
                anio:self.anio,
                precio_bruto: self.precio_bruto,
                color: self.color.clone(),
            }
        }
    }


fn main(){
    let mut concesionariaQuilmes = ConcesionarioAuto::new("Concesionaria de Quilmes".to_string(), "Calle Balverde 1243".to_string(), 20);
    let auto1 = Auto::new("BMW".to_string(), "SUV".to_string(),1998,43000.50, Color::AMARILLO);
    let auto2 = Auto::new("Mercedez Benz".to_string(), "Deportivo".to_string(),2005,29000.40, Color::ROJO);
    let auto3 = Auto::new("Fiat".to_string(), "Uno".to_string(),2015,20000.00, Color::AZUL);
    concesionariaQuilmes.agregar_auto(auto1);
    concesionariaQuilmes.agregar_auto(auto2);
    concesionariaQuilmes.agregar_auto(auto3);
    for i in 0..3{
        println!("Esta el auto de marca: {}", concesionariaQuilmes.autos[i as usize].marca);
    }
}