use horizon_prelude::*;

pub struct ViewModelSettings {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub scale: f32,
}

impl Default for ViewModelSettings {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            scale: 1.0,
        }
    }
}

static mut SETTINGS: ViewModelSettings = ViewModelSettings {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    scale: 1.0,
};

#[horizon_mod_init]
pub fn init() {
    ui::register_settings_menu("View Model Customizer", render_settings_ui);
    
    graphics::on_render_item(|matrix| {
        unsafe {
            matrix.translate(SETTINGS.x, SETTINGS.y, SETTINGS.z);
            matrix.scale(SETTINGS.scale, SETTINGS.scale, SETTINGS.scale);
        }
    });
}

fn render_settings_ui(ui: &mut ui::Ui) {
    ui.heading("Ustawienia View Modelu");
    
    unsafe {
        ui.slider("Pozycja X (Lewo / Prawo)", &mut SETTINGS.x, -1.0, 1.0);
        ui.slider("Pozycja Y (Góra / Dół)", &mut SETTINGS.y, -1.0, 1.0);
        ui.slider("Pozycja Z (Przód / Tył)", &mut SETTINGS.z, -2.0, 2.0);
        ui.slider("Skala przedmiotu", &mut SETTINGS.scale, 0.5, 2.0);
        
        if ui.button("Resetuj domyślne") {
            SETTINGS.x = 0.0;
            SETTINGS.y = 0.0;
            SETTINGS.z = 0.0;
            SETTINGS.scale = 1.0;
        }
    }
}