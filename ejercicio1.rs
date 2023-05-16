struct Persona{
    nombre:String,
    edad:i32,
    direccion:Option<String>,
    
}
impl Persona{

 fn new( nombre:String, edad:i32, direccion:Option<String> ) -> Persona{
     Persona{
             nombre,
             edad,
             direccion,
         }
        }
         fn imprimir(&self){
            println!("El nombre de la persona es {}", self.nombre);
            println!("Vive en {}",self.edad);
            match &self.direccion{
                Some(a) =>println!("Su direccion es {}", a),
                None =>println!("No sabemos donde vive"),
            }
        }
        
        fn obtener_edad(&self) -> i32{
            return self.edad
        }
        
        fn actualizar_direccion(&mut self, nueva_direccion:String){
            self.direccion = Some(nueva_direccion);
        }
}

fn main(){
    let mut persona1 = Persona::new("Juan".to_string(), 32, None);
    persona1.imprimir();
    println!("La edad de la persona es {}", persona1.obtener_edad());
    persona1.actualizar_direccion("LUGAR".to_string());
    persona1.imprimir();
}