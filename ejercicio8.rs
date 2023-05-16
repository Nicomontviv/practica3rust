#[derive(Debug)]
#[derive(PartialEq)]
use std::convert::TryInto;
enum Genero{
    ROCK,
    POP,
    JAZZ,
    RAP,
    OTRO,
}

impl Eq for Genero {}

impl Genero {
    fn Eq(&self, other:Genero) -> bool {
        self == other
    }
}
#[derive(PartialEq)]
struct Cancion{
    titulo:String,
    artista:String,
    genero:Genero,
}
impl Eq for Cancion {}
impl Cancion{
    fn new(titulo:String, artista:String, genero:Genero) -> Cancion{
        Cancion{
            titulo,
            artista,
            genero,
        }
    }
        fn clonar(&self) -> Cancion{
            Cancion{
                titulo:self.titulo,
                artista:self.artista,
                genero: self.genero,
            }
        }
        fn Eq(&self, otro:&Cancion) -> bool{
            if self == otro {
                return true
            }
            else{
                return false
            }
        }
    }

struct Playlist{
    titulo:String,
    canciones:Vec<Cancion>,
}
impl Playlist{
    fn new(titulo:String) ->Playlist{
        Playlist{
            titulo,
            canciones:Vec::new(),
        }
    }
    fn agregar_cancion(&mut self, cancion:Cancion){
         self.canciones.push(cancion);
    }
    fn eliminar_cancion(&mut self, cancion:Cancion) -> bool{
        let mut eliminado:bool = false;
        let mut i:usize = 0;
        while i < self.canciones.len() && !self.canciones[i].Eq(&cancion){
            i = i + 1;
        }
        if self.canciones[i].Eq(&cancion){
            self.canciones.remove(i);
            eliminado = true;
        }
        return eliminado
    }
    fn mover_cancion(&mut self, cancion:Cancion, pos:i32) ->bool{
        let mut encontrado:bool = false;
        if pos > self.canciones.len(){
            return encontrado 
        }else {
        let mut i:i32 = -1;
        while i < self.canciones.len().try_into().unwrap() && encontrado != true {
              i = i + 1;
              if self.canciones[i as usize].titulo == cancion.titulo && self.canciones[i as usize].artista == cancion.artista{
                encontrado = true;
              }
        }
        if encontrado == false{ return false} else{
        let la_cancion:Cancion = self.canciones.remove(i as usize);
        self.canciones.insert(pos as usize, la_cancion);
        return true 
        }
       }
    }
    
    fn buscar_por_nombre(&self, unNombre:String) -> Option<&Cancion>{
        let mut i:usize = 0;
        while i< self.canciones.len() && self.canciones[i].titulo!= unNombre{
            i = i + 1;
        } 
        if self.canciones[i].titulo == unNombre{
            return Some(&self.canciones[i]) }else{
            return None
            
        }
    }
    fn obtener_por_genero(&self, unGenero:Genero) -> Vec<Cancion>{
            let mut canciones_genero = Vec::new();
            for i in 0..self.canciones.len(){
                if self.canciones[i].genero.Eq(&unGenero) {
                          canciones_genero.push(self.canciones[i].clonar());
                }
            }
            return canciones_genero
    }
    fn obtener_por_artista(&self, unArtista:String) -> Vec<&Cancion>{
        let mut canciones_artista = Vec::new();
        for i in 0..self.canciones.len(){
            if self.canciones[i].artista == unArtista {
                      canciones_artista.push(&self.canciones[i]);
            }
        }
        return canciones_artista
    }
    fn modificar_titulo(&mut self, nuevoTitulo:String){
        self.titulo = nuevoTitulo;
    }
    fn eliminar_todas(&mut self){
        while self.canciones.len() != 0{
            self.canciones.pop();
        }
    }
}

fn main(){
    let nueva_playlist1 = Playlist::new("mis_canciones".to_string());
}




