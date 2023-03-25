#include "azusa.h"

int main() {
    Azusa *azusa = azusa_new();
    Surface *surface = azusa_new_image_surface("c-example.png",200,200);
    Color white = { 255,255,255,255 };
    Color black = { 0,0,0,255 };

    azusa_clear(azusa,white);
    azusa_draw_rectangle(azusa,black,199,199,1);
    azusa_move_to(azusa,1,1);

    azusa_flush(azusa,surface);
    return 0;
}