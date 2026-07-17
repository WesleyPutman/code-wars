fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {		//retourne false si la longueur du tableau walk n'est pas égale à 10
        return false;
    }
    let mut x = 0; 				//mut permet de modifier la valeur de x qui est pas mutable par défaut
    let mut y = 0;
    
    for direction in walk{		//boucle for qui parcourt le tableau walk de façon orthonormé 
        match direction { 		//match permet de faire un switch case sur la variable direction
            'n' => y += 1,
            's' => y -= 1,
            'e' => x += 1,
            'w' => x -= 1,
            _ => (),
        }
    }
    x == 0 && y == 0			//retourne true si on retourne au point de départ, sinon false
}
