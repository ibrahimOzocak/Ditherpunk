use argh::FromArgs;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}
 
const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);

use image::{ImageError, RgbImage, RgbaImage, Rgba, Rgb};

fn main() -> Result<(), ImageError>{
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
     
    // Image to image RGB
    let img = image::open(&path_in)?;
    let img_rgb = img.to_rgb8();
    img_rgb.save("img/question3.png")?;
    println!("Image Convertie en RGB");

    // Afficher couleur pixel (32, 52)
    let pixel = img_rgb.get_pixel(32, 52);
    println!("Couleur du pixel (32, 52) : {:?}", pixel);

    //  un pixel sur deux d’une image en blanc
    let img2 = image::open("img/image.jpg")?;
    let mut img2_rgb = img2.to_rgb8();
    for y in 0..img2_rgb.height() {
        for x in 0..img2_rgb.width() {
            if (x + y) % 2 == 0 {
                img2_rgb.put_pixel(x, y, WHITE);
            }
        }
    }
    img2_rgb.save("img/question5.jpg")?;


    // Créer une nouvelle image pour les pixels monochromes
    let mut img_monochrome: RgbaImage = RgbaImage::new(img2_rgb.width(), img2_rgb.height());

    // Seuillage et conversion en monochrome
    for (x, y, pixel) in img_rgb.enumerate_pixels() {
        // Accéder directement aux valeurs R, G, B
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
        
        // Calculer la luminosité
        let luminosity = 0.2989 * r as f32 + 0.5870 * g as f32 + 0.1140 * b as f32;

        // Appliquer le seuillage
        let color = if luminosity > 127.0 { 255 } else { 0 };  // Seuil à 50% de 255

        // Assigner la couleur du pixel en monochrome
        img_monochrome.put_pixel(x, y, Rgba([color, color, color, 255]));
    }
    
    // Sauvegarder l'image résultante
    img_monochrome.save("img/question7.png")?;
    Ok(())
}
