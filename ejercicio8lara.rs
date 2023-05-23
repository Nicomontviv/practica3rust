pub fn ejercicio8(){
    /*  8- Defina la estructura Cancion con campos para el título, el artista y el género. El género puede
        ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta por una
        lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre ella:
            ➔ agregar canción. (*)
            ➔ eliminar canción. (*)
            ➔ mover canción // mueve la canción a una determinada posición de la playlist. (*)
            ➔ buscar canción por nombre. (*)
            ➔ obtener las canciones de un determinado género. (*)
            ➔ obtener las canciones de un determinado artista. (*)
            ➔ modificar título de la playlist. (*)
            ➔ eliminar todas las canciones.
    */
        #[derive(Debug)]
        struct Cancion{
            titulo: String,
            artista: String,
            genero: Genero,
        }
        impl Cancion {
            pub fn new (titulo:String, artista:String, genero:Genero) -> Cancion {
                Cancion { titulo, artista, genero }
            }        
            pub fn misma_cancion (&self, cancion:&Cancion) -> bool {
                return self.artista == cancion.artista && self.titulo == cancion.titulo
                && self.genero.equals(&cancion.genero);
            }
            pub fn mismo_titulo(&self, nombre:&String) -> bool {
                return self.titulo == *nombre;
            }
            pub fn mismo_artista(&self, nombre:&String) -> bool {
                return self.artista == *nombre;
            }
            pub fn copiar(&self) -> Cancion {
                let t:String = self.titulo.clone();
                let a:String = self.artista.clone();
                let mut g:Genero;
                match self.genero{
                    Genero::Jazz => g=Genero::Jazz,
                    Genero::Otro=> g=Genero::Otro,
                    Genero::Pop=> g=Genero::Pop,
                    Genero::Rock=> g=Genero::Rock,
                    Genero::Rap=> g=Genero::Rap,
                } 
                Cancion { titulo: t, artista: a, genero: g }
            }
        }
        #[derive(Debug)]
        enum Genero{
            Rock,
            Pop,
            Rap,
            Jazz,
            Otro,
        }
        impl Genero{
            pub fn equals(&self, otro_genero:&Genero) -> bool {
                match self{
                    Genero::Jazz => match otro_genero{
                        Genero::Jazz => return true,
                        _ => return false,
                    }
                    Genero::Otro => match otro_genero {
                        Genero::Otro => return true,
                        _ => return false,
                    }
                    Genero::Pop => match otro_genero {
                        Genero::Pop => return true,
                        _ => return false,
                    }
                    Genero::Rap => match otro_genero {
                        Genero::Rap => return true,
                        _ => return false,
                    }
                    Genero::Rock => match otro_genero {
                        Genero::Rock => return true,
                        _ => return false,
                    }
                }
                
            }
        }
        #[derive(Debug)]
        struct Playlist{
            titulo_de_la_playlist: String,
            lista: Vec<Cancion>,
        }
        impl Playlist {
            pub fn new (nombre:String, vec_pl:Vec<Cancion>) -> Playlist{
                Playlist { titulo_de_la_playlist: nombre, lista: vec_pl }
            }
            pub fn agregar_cancion(&mut self, cancion:Cancion){
                self.lista.push(cancion);
            }
            pub fn buscar_indice(&self, cancion:&Cancion) -> Option<usize>{
                for i in 0..self.lista.len() {
                    if self.lista[i].misma_cancion(&cancion){
                        return Some(i);
                    }
                }
                return None;
            }
            pub fn eliminar_cancion(&mut self, cancion:Cancion) -> bool {
                match self.buscar_indice(&cancion){
                    Some(indice) => { self.lista.remove(indice);
                    return true;}
                    _ => return false,
                }
            }
            pub fn mover_canción(&mut self, cancion:Cancion ,posicion:usize) -> bool{
                if let Some(indice_cancion) = self.buscar_indice(&cancion){ //la cancion existe en el vec
                    if posicion < self.lista.len(){ //la posicion es valida
                        self.lista.remove(indice_cancion);
                        self.lista.insert(posicion, cancion);
                        return true;
                    }else{
                        return false;
                    }
                }else{
                    return false;
                }
            }   
            pub fn buscar_canción_por_nombre (&self, nombre:String) -> Option<Cancion>{
                for i in 0..self.lista.len() {
                    if self.lista[i].mismo_titulo(&nombre){
                        return Some(self.lista[i].copiar());
                    }
                }
                return None;
            }
            pub fn obtener_canciones_por_genero (&self, genero:Genero) -> Vec<Cancion> {
                let mut playlist_genero:Vec<Cancion>=Vec::new();
                for i in 0..self.lista.len() {
                    if self.lista[i].genero.equals(&genero){
                        playlist_genero.push(self.lista[i].copiar());
                    }
                }
                return playlist_genero;
            }
            //obtener las canciones de un determinado artista
            pub fn obtener_canciones_por_artista (&self, artista:String) -> Vec<Cancion> {
                let mut playlist_genero:Vec<Cancion>=Vec::new();
                for i in 0..self.lista.len() {
                    if self.lista[i].mismo_artista(&artista){
                        playlist_genero.push(self.lista[i].copiar());
                    }
                }
                return playlist_genero;
            }
            pub fn cambiar_titulo (&mut self, nuevo_titulo:String){
                self.titulo_de_la_playlist=nuevo_titulo;
            }
            pub fn borrar_todo(&mut self){
                self.lista.clear();
            }
    ​
    ​
        }
    ​
        //main test
        let vec_pl: Vec<Cancion> = Vec::new();
        let mut playlist_1:Playlist = Playlist::new("Playlist 1".to_string(), vec_pl);
        let cancion_1:Cancion = Cancion::new("Words of Wisdom".to_string(),
         "2Pac".to_string(), Genero::Rap);
        playlist_1.agregar_cancion(cancion_1);
        println!("{:?}", playlist_1);
        println!("...");
    ​
        println!("Eliminar");
        let cancion_eliminar:Cancion = Cancion::new("Words of Wisdom".to_string(),
         "2Pac".to_string(), Genero::Rap);
        println!("{:?}", playlist_1.eliminar_cancion(cancion_eliminar));
    ​
        let cancion_eliminar_2:Cancion = Cancion::new("Hit 'em up".to_string(),
         "2Pac".to_string(), Genero::Rap);
        println!("{:?}", playlist_1.eliminar_cancion(cancion_eliminar_2));
    ​
        let cancion_eliminar_3:Cancion = Cancion::new("Hit 'em up".to_string(),
         "2Pac".to_string(), Genero::Jazz);
        println!("{:?}", playlist_1.eliminar_cancion(cancion_eliminar_3));
    ​
        println!("...");
        let cancion_2:Cancion = Cancion::new("Bitches Brew".to_string(),
         "Miles Davis".to_string(), Genero::Jazz);
        playlist_1.agregar_cancion(cancion_2);
        let cancion_3:Cancion = Cancion::new("When Doves Cry".to_string(),
         "Prince".to_string(), Genero::Pop);
        playlist_1.agregar_cancion(cancion_3);
        let cancion_4:Cancion = Cancion::new("Dolly Dagger".to_string(),
         "Jimi Hendrix".to_string(), Genero::Rock);
        playlist_1.agregar_cancion(cancion_4);
        let cancion_5:Cancion = Cancion::new("Crosstown Traffic".to_string(),
         "Jimi Hendrix".to_string(), Genero::Rock);
        playlist_1.agregar_cancion(cancion_5);
        let cancion_6:Cancion = Cancion::new("Computer Blues".to_string(),
         "Prince".to_string(), Genero::Otro);
         playlist_1.agregar_cancion(cancion_6);
         let cancion_7:Cancion = Cancion::new("Paisley Park".to_string(),
         "Prince".to_string(), Genero::Rock);
         playlist_1.agregar_cancion(cancion_7);
        println!("{:?}", playlist_1);
    ​
        let cancion_5_repetida:Cancion = Cancion::new("Crosstown Traffic".to_string(),
         "Jimi Hendrix".to_string(), Genero::Rock);
        playlist_1.mover_canción(cancion_5_repetida, 0);
        println!("...");
        println!("{:?}", playlist_1);
    ​
        println!("...");
        let palomas:String = "When Doves Cry".to_string();
        println!("Buscar {}",palomas);
        println!("{:?}", playlist_1.buscar_canción_por_nombre(palomas));
    ​
        println!("...");
        println!("obtener las canciones de un determinado género");
        println!("{:?}", playlist_1.obtener_canciones_por_genero(Genero::Rock));
    ​
        println!("...");
        println!("obtener las canciones de un determinado artista");
        println!("{:?}", playlist_1.obtener_canciones_por_artista("Prince".to_string()));
    ​
        playlist_1.cambiar_titulo("Los mas grandes".to_string());
        println!("{:?}", playlist_1);
        playlist_1.borrar_todo();
        println!("{:?}", playlist_1);
    ​
    }