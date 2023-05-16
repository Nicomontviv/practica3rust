use std::collections::VecDeque;

struct Examen{
    materia:String,
    nota:f64,
}
struct Estudiante{
    nombre:String,
    id:i32,
    examenes:VecDeque<Examen>, 
}

impl Examen{
    fn new(unaMateria:String, unaNota:f64) -> Examen{
        Examen{
           materia:unaMateria,
           nota:unaNota,
        }
    }
}
impl Estudiante{
   
    fn new(unNombre:String, unId:i32) -> Estudiante{
        Estudiante{
            nombre:unNombre,
            id:unId,
            examenes:VecDeque::new(),
        }
    }

    fn agregar_examen(&mut self, examen:Examen){
        self.examenes.push_front(examen);
    }
    fn obtener_promedio(&self) -> f64{
        if self.examenes.len() == 0 {
            return 0.0
        } else{
            let mut total_notas:f64 = 0.0;
            let mut cant:i32 = 0;
            for i in 0..self.examenes.len(){
                total_notas = total_notas + self.examenes[i].nota;
                cant = cant + 1;
            }
            return total_notas/cant as f64
        }

    }
    fn obtener_calificacion_mas_alta(&self) -> f64{
          let mut nota_mas_alta:f64 = 0.0;
          for i in 0..self.examenes.len(){
            if self.examenes[i].nota > nota_mas_alta{
                nota_mas_alta = self.examenes[i].nota;
            }
          }
          return nota_mas_alta
    }
    fn obtener_calificacion_mas_baja(&self) ->f64{
          let mut nota_mas_baja:f64 = 10.0;
          for i in 0..self.examenes.len(){
            if self.examenes[i].nota < nota_mas_baja{
                nota_mas_baja= self.examenes[i].nota;
            }
          }
          return nota_mas_baja
    }
    fn estudiante_to_string(&self){
       println!("El estudiante se llama {}", self.nombre);
       match self.examenes.len(){
       0 => println!("Aun no ha tenido examenes"),
       other => println!("El promedio es de {}", self.obtener_promedio()),
       }
    }

}
fn main(){
    let mut estudiante1 = Estudiante::new("Marcos".to_string(), 2087);
    let examen_matematica = Examen::new("Matematica".to_string(), 8.6);
    estudiante1.agregar_examen(examen_matematica);
    let examen_logica = Examen::new("Logica".to_string(), 5.0);
    estudiante1.agregar_examen(examen_logica);
    let mut estudiante2 = Estudiante::new("Joaquin".to_string(), 1304);
    println!("El estudiante {} tiene como nota mas alta un {}",estudiante1.nombre, estudiante1.obtener_calificacion_mas_alta());
    println!("Y el promedio es {}",estudiante1.obtener_promedio());
    estudiante2.estudiante_to_string();
    let examen_matematica2 = Examen::new("Matematica".to_string(), 7.0);
    estudiante2.agregar_examen(examen_matematica2);
    estudiante2.estudiante_to_string();
}