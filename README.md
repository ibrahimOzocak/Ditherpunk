# Ditherpunk: retour au monochrome

## 1 La bibliothèque image

### Question 2

- Pour ouvrir une image depuis un fichier, on utilise la fonction image::open(), qui retourne un type DynamicImage. 

- Pour obtenir l'image au format RGB, on utilise la fonction to_rgb8 sur l'image récupérer

let img_rgb = img.to_rgb8();

Pour convertir l'image, on utilise la ligne de commande :

cargo run -- img/img1.png seuil


### Question 3

Pour save l'image, on utilise :

img_rgb.save("img/question3.png")?;

Avant :

<img src="ditherpunk/img/img1.png"></img>

Après :

<img src="ditherpunk/img/question3.png"></img>

On peut que sur l'image de base avec un canal alpha (transaparence), cette transparence sera perdue lorsque l’image est convertie en Rgb8. Cela signifie que toute zone transparente dans l'image d'origine sera rendue avec une couleur  généralement noire, car le format Rgb8 ne gère pas les canaux alpha.